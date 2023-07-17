use std::default;

use super::token::get_staff_token;
use actix_web::web;
use awc::error::SendRequestError;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HiddenUser {
    Invisible(String),
    User(User),
    #[default]
    None
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HiddenUsers {
    Invisible(String),
    Users(Vec<User>),
    #[default]
    None
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScaleTeam {
    pub id: i32,
    pub begin_at: chrono::DateTime<Utc>,
    pub correcteds: HiddenUsers,
    pub corrector: HiddenUser,
    pub filled_at: Option<chrono::DateTime<Utc>>,
    pub team: Team,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub login: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    pub project_gitlab_path: String,
}

pub async fn get_scale_team(
    id: i32,
    client: web::Data<awc::Client>,
    auth_client: web::Data<oauth2::basic::BasicClient>,
) -> Result<ScaleTeam, SendRequestError> {
    let team = client
        .get(format!("https://api.intra.42.fr/v2/scale_teams/{id}"))
        .bearer_auth(get_staff_token(auth_client).await?.secret())
        .send()
        .await?
        .json::<ScaleTeam>()
        .await
        .unwrap();
    Ok(team)
}
