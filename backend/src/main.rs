// #![allow(dead_code)]
mod api;
mod api_42;
mod db;
mod permission;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::middleware::Logger;
use actix_web::{cookie, web, App, HttpServer};
use actix_web_grants::{GrantsMiddleware, PermissionGuard};
use db::migration::run_migration;
use db::Database;
use permission::extract_permission;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Permission {
    Staff,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().expect("Failed to load environment");
    env_logger::init();

    let db_url = &std::env::var("DATABASE_URL").expect("env not set: DATABASE_URL");

    run_migration(&db_url).expect("Failed to run migration");

    let port: u16 = std::env::var("BACKEND_PORT")
        .expect("env not set: BACKEND_PORT")
        .parse::<u16>()
        .expect("Invalid BACKEND_PORT");

    let db = Database::new(&db_url)
        .await
        .expect("Failed to create new database");

    HttpServer::new(move || {
        let key = std::env::var("SESSION_KEY").expect("env not set: SESSION_KEY");
        let key = cookie::Key::from(key.as_bytes());
        let auth_client = new_auth_client();
        let client = awc::Client::default();
        let cors = Cors::default()
            .allow_any_origin()
            // .allowed_origin(&std::env::var("FRONTEND_URL").expect("env not set: FRONTEND_URL"))
            // .allowed_origin("https://profile.intra.42.fr")
            // .allowed_origin("https://api.intra.42.fr")
            .supports_credentials()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(auth_client))
            .app_data(web::Data::new(client))
            .app_data(web::JsonConfig::default().limit(16384))
            .app_data(web::PayloadConfig::new(16384))
            .wrap(GrantsMiddleware::with_extractor(extract_permission))
            .wrap(IdentityMiddleware::builder().build())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .wrap(Logger::default())
            .service(
                web::scope("/api/admin")
                    .configure(api::admin::init)
                    .guard(PermissionGuard::new(Permission::Staff)),
            )
            // .service(
            //     web::resource("/api/admin/{tail:.*}") // fallback endpoint to return 403 Forbidden
            //         .to(|| async { HttpResponse::Forbidden().body("Forbidden") }),
            // )
            .service(
                web::scope("/api")
                    .configure(api::auth::init)
                    .configure(api::feedback::init)
                    .configure(api::evaluation::init)
                    .configure(api::ping::init), // .configure(api::typeform::init),
            )
    })
    .bind(("0.0.0.0", port))
    .expect(&format!("Failed to bind to port {port}"))
    .run()
    .await
    .expect("Failed to run server");
    Ok(())
}

pub fn new_auth_client() -> oauth2::basic::BasicClient {
    oauth2::basic::BasicClient::new(
        oauth2::ClientId::new(std::env::var("CLIENT_ID").expect("env not set: CLIENT_ID")),
        Some(oauth2::ClientSecret::new(
            std::env::var("CLIENT_SECRET").expect("env not set: CLIENT_SECRET"),
        )),
        oauth2::AuthUrl::new("https://api.intra.42.fr/oauth/authorize".to_string())
            .expect("Invalid authorization endpoint URL (AUTH_URL)"),
        Some(
            oauth2::TokenUrl::new("https://api.intra.42.fr/oauth/token".to_string())
                .expect("Invalid token endpoint URL (TOKEN_URL)"),
        ),
    )
    .set_redirect_uri(
        oauth2::RedirectUrl::new(std::env::var("REDIRECT_URL").expect("env not set: REDIRECT_URL"))
            .expect("Invalid redirect URL"),
    )
}
