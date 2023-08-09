use super::error::ApiError;
use crate::api_42::team::get_team;
use crate::db::enums::FeedbackKind;
use crate::db::model::{NewEvaluation, NewEvaluationFeedback};
use crate::db::Database;
use actix_web::HttpRequest;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/evaluation")
            .service(web::resource("/").route(web::post().to(update_callback)))
    );
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScaleTeam {
    pub id: i32,
    pub team: Team,
    pub begin_at: chrono::DateTime<Utc>,
    pub final_mark: Option<i32>,
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

async fn update_callback(
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
    if std::env::var("WEBHOOK_SECRET_UPDATE")
        .expect("WEBHOOK_SECRET_UPDATE")
        .ne(secret)
    {
        return Err(ApiError::Unauthorized);
    }
    if evaluation.final_mark.is_none() {
        log::debug!("evaluation not finished");
        return Ok(HttpResponse::Ok().finish());
    }
    if db.get_evaluation(evaluation.id).await.is_ok() {
        log::debug!("evaluation already added");
        return Ok(HttpResponse::Ok().finish());
    }
    let new_evaluation = NewEvaluation {
        scale_team_id: evaluation.id,
        team: evaluation.team.name.to_owned(),
        project: evaluation.project.name.to_owned(),
        evaluator_id: evaluation.user.id,
        begin_at: evaluation.begin_at.naive_utc(),
    };
    let team = match get_team(evaluation.team.id, client, auth_client).await {
        Ok(team) => team,
        Err(_) => return Err(ApiError::InternalServerError),
    };
    log::info!("team: {:?}", team);
    let new_evaluation = db.add_evaluation(new_evaluation).await?;
    log::info!("added evaluation: {:?}", new_evaluation);

    for user_id in team.users.iter().map(|u| u.id) {
        let new_feedback = NewEvaluationFeedback {
            evaluation_id: new_evaluation.id,
            user_id,
            kind: FeedbackKind::Evaluated,
        };
        log::debug!(
            "added evaluation feedback for evaluated: {:?}",
            new_feedback
        );
        db.add_evaluation_feedback(new_feedback).await?;
    }
    {
        let new_feedback = NewEvaluationFeedback {
            evaluation_id: new_evaluation.id,
            user_id: new_evaluation.evaluator_id,
            kind: FeedbackKind::Evaluator,
        };
        log::debug!(
            "added evaluation feedback for evaluator: {:?}",
            new_feedback
        );
        db.add_evaluation_feedback(new_feedback).await?;
    }
    return Ok(HttpResponse::Ok().finish());
}
