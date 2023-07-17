use super::error::ApiError;
use crate::api_42::scale_team::get_scale_team;
use crate::db::model::{NewEvaluation, NewEvaluationFeedback};
use crate::db::Database;
use actix_web::HttpRequest;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/evaluation")
            .service(web::resource("/").route(web::post().to(add_evauation)))
            .service(web::resource("/delete").route(web::post().to(delete_evauation))),
    );
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScaleTeam {
    pub id: i32,
    pub team: Team,
    pub begin_at: chrono::DateTime<Utc>,
    pub project: Project,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub id: i32,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub login: String,
}

async fn add_evauation(
    db: web::Data<Database>,
    evaluation: web::Json<ScaleTeam>,
    client: web::Data<awc::Client>,
    auth_client: web::Data<oauth2::basic::BasicClient>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let secret = req
        .headers()
        .get("x-secret")
        .ok_or(ApiError::Unauthorized)?;
    if std::env::var("WEBHOOK_SECRET")
        .expect("WEBHOOK_SECRET")
        .ne(secret)
    {
        return Err(ApiError::Unauthorized);
    }
    let new_evaluation = NewEvaluation {
        scale_team_id: evaluation.id,
        team: evaluation.team.name.to_owned(),
        project: evaluation.project.name.to_owned(),
        evaluator_id: evaluation.user.id,
        begin_at: evaluation.begin_at.naive_utc(),
    };
    let team = match get_scale_team(evaluation.team.id, client, auth_client).await {
        Ok(team) => team,
        Err(_) => return Err(ApiError::InternalServerError),
    };
    log::info!("team: {:?}", team);
    let new_evaluation = db.add_evaluation(new_evaluation).await?;
    log::info!("added evaluation: {:?}", new_evaluation);
    let mut user_ids: Vec<i32> = team.correcteds.iter().map(|u| u.id).collect();
    user_ids.push(team.corrector.id);
    log::info!("added evaluation_users: {:?}", user_ids);
    for user_id in user_ids {
        let new_feedback = NewEvaluationFeedback {
            evaluation_id: new_evaluation.id,
            user_id: user_id,
        };
        log::debug!("added evaluation feedback: {:?}", new_feedback);
        db.add_evaluation_feedback(new_feedback).await?;
    }
    return Ok(HttpResponse::Ok().finish());
}

async fn delete_evauation(
    db: web::Data<Database>,
    evaluation: web::Json<ScaleTeam>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let secret = req
        .headers()
        .get("x-secret")
        .ok_or(ApiError::Unauthorized)?;
    if std::env::var("WEBHOOK_SECRET")
        .expect("WEBHOOK_SECRET")
        .ne(secret)
    {
        return Err(ApiError::Unauthorized);
    }
    log::info!("evaluation removed: {:?}", evaluation);
    db.delete_evaluation(evaluation.id).await?;
    return Ok(HttpResponse::Ok().finish());
}
