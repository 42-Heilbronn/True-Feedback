use actix_web::web;
use awc::error::SendRequestError;
use oauth2::{reqwest::async_http_client, Scope, TokenResponse};

pub async fn get_staff_token(
    auth_client: web::Data<oauth2::basic::BasicClient>,
) -> Result<oauth2::AccessToken, SendRequestError> {
    match auth_client
        .exchange_client_credentials()
        .add_scope(Scope::new("public".to_string()))
        .request_async(async_http_client)
        .await
    {
        Ok(token) => Ok(token.access_token().clone()),
        Err(_) => Err(SendRequestError::Timeout),
    }
}
