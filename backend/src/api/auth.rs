use crate::permission::Permission;

use super::error::ApiError;
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse};
use oauth2::{PkceCodeVerifier, TokenResponse};
use serde::Deserialize;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::get().to(login)))
        .service(web::resource("/callback").route(web::get().to(ft_callback)));
}

#[derive(Deserialize)]
pub struct LoginRequest {
    redirect: Option<String>,
    no_redirect: Option<bool>,
}

async fn login(
    id: Option<Identity>,
    session: Session,
    auth_client: web::Data<oauth2::basic::BasicClient>,
    query: web::Query<LoginRequest>,
) -> Result<HttpResponse, ApiError> {
    log::debug!("login attempt");

    // If user is already logged in redirect to root
    if let Some(id) = id {
        log::debug!("user {:?} already logged in", id.id());
        return Ok(HttpResponse::Found()
            .insert_header((
                header::LOCATION,
                std::env::var("FRONTEND_URL").expect("FRONTEND_URL not set"),
            ))
            .finish());
    }
    // Build authorize_url the user will be redirected to
    let (pkce_code_challenge, pkce_code_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
    let (authorize_url, csrf_state) = &auth_client
        .authorize_url(oauth2::CsrfToken::new_random)
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    // Store pkce_verifier and state in session for CSRF protection
    session.insert("pkce_verifier", pkce_code_verifier)?;
    session.insert("state", csrf_state.secret())?;
    if query.redirect.is_some() {
        let redirect_url = query.redirect.as_ref().unwrap();
        session.insert("redirect_url", redirect_url)?;
    }
    if query.no_redirect.unwrap_or(false) {
        return Ok(HttpResponse::Ok().body(authorize_url.to_string()));
    }
    Ok(HttpResponse::Found().insert_header((header::LOCATION, authorize_url.to_string())).finish())
}

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

async fn ft_callback(
    id: Option<Identity>,
    session: Session,
    request: HttpRequest,
    auth_client: web::Data<oauth2::basic::BasicClient>,
    params: web::Query<AuthRequest>,
) -> Result<HttpResponse, ApiError> {
    let redirect_url = session.get::<String>("redirect_url")?;
    // If user is already logged in redirect to frontend
    if id.is_some() {
        log::debug!("callback user {:?} already logged in", id.unwrap().id());
        return Ok(HttpResponse::Found()
            .insert_header((
                header::LOCATION,
                redirect_url
                    .unwrap_or(std::env::var("FRONTEND_URL").expect("FRONTEND_URL not set")),
            ))
            .finish());
    }

    let code = oauth2::AuthorizationCode::new(params.code.clone());
    let state = oauth2::CsrfToken::new(params.state.clone());

    // Verify the state for CSRF protection
    let session_state = session.get::<String>("state")?;
    if session_state.is_none() {
        log::info!("callback without state");
        return Err(ApiError::BadRequest("No state".to_string()));
    }
    if session_state.unwrap() != *state.secret() {
        log::info!("callback with invalid state");
        return Err(ApiError::BadRequest("Invalid state".to_string()));
    }

    // Exchange the code from the user with a token from 42 intra
    let Some(pkce_verifier) = session.get::<PkceCodeVerifier>("pkce_verifier")? else {
        log::info!("Failed to get pkce_verifier from session");
        return Err(ApiError::InternalServerError);
    };

    let token = &auth_client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(oauth2::reqwest::async_http_client)
        .await;

    let token = match token {
        Ok(token) => token,
        Err(e) => {
            log::error!("Failed to exchange token with 42 Intra: {}", e);
            return Err(ApiError::BadRequest(format!(
                "Failed to exchange token with 42 Intra: {}",
                e
            )));
        }
    };

    // Remove old session data
    // And save the token from 42 Intra in the session.
    session.remove("pkce_verifier");
    session.remove("state");
    session.insert("token", token)?;

    // Get the user's information from 42 Intra
    let client = awc::Client::default();

    let Ok(mut res) = client
        .get("https://api.intra.42.fr/v2/me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await else {
            log::error!("Failed to get user info from 42 Intra");
            return Err(ApiError::InternalServerError);
        };

    // Parse the response from 42 Intra
    let Ok(data) = res.json::<serde_json::Value>().await else {
        log::error!("Failed to parse user info from 42 Intra");
        return Err(ApiError::InternalServerError);
    };

    let id: i32 = data["id"].as_i64().unwrap().try_into().unwrap();
    let login = match data["login"].as_str() {
        Some(login) => login,
        None => {
            log::error!("Failed to extract user info from 42 Intra",);
            return Err(ApiError::InternalServerError);
        }
    };

    if data["staff?"].as_bool().is_some_and(|x| x == true) {
        session.insert("permission", vec![Permission::Staff])?;
    }

    session.insert("user_id", id)?;

    Identity::login(&request.extensions(), id.to_string())?;
    log::info!("Logged in user: {} with id {}", login, id);

    return Ok(HttpResponse::Found()
        .insert_header((
            header::LOCATION,
            redirect_url.unwrap_or(std::env::var("FRONTEND_URL").expect("FRONTEND_URL not set")),
        ))
        .finish());
}
