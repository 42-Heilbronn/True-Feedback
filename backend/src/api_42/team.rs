use super::token::get_staff_token;
use actix_web::web;
use awc::error::SendRequestError;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub users: Vec<User>,
    pub project_gitlab_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub login: String,
}

pub async fn get_team(
    id: i32,
    client: web::Data<awc::Client>,
    auth_client: web::Data<oauth2::basic::BasicClient>,
) -> Result<Team, SendRequestError> {
    let team: Team = client
        .get(format!("https://api.intra.42.fr/v2/teams/{id}"))
        .bearer_auth(get_staff_token(auth_client).await?.secret())
        .send()
        .await?
        .json::<Team>()
        .await
        .expect("team format");
    Ok(team)
}
