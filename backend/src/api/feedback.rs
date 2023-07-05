use super::feedback_structure::{FeedbackEvaluator, FeedbackStructureField, FEEDBACK_EVALUATOR_FIELDS};
use crate::{api_42::scale_team::get_scale_team};
use crate::db::Database;
use actix_identity::Identity;
// use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::{Serialize};

use super::error::ApiError;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/feedback")
            .service(web::resource("/missing").route(web::get().to(missing_feedback)))
            .service(
                web::resource("/{feedback_id}/info").route(web::get().to(evaluation_feedback_info)),
            )
            .service(web::resource("/{feedback_id}").route(web::post().to(post_feedback))),
    );
}

#[derive(Serialize)]
struct FeedbackListEntry {
    pub id: i32,
    pub evaluation: FeedbackListEvaluationEnrty,
}
#[derive(Serialize)]
struct FeedbackListEvaluationEnrty {
    pub team: String,
    pub project: String,
    pub begin_at: chrono::NaiveDateTime,
}

async fn missing_feedback(
    id: Identity,
    db: web::Data<Database>,
) -> Result<HttpResponse, ApiError> {
    let user_id: i32 = id.id().unwrap().parse::<i32>().unwrap();
    // log::warn!("DEBUG USER ID NO IDENTITY!");
    let missing_feedback = db.get_missing_evaluation_feedbacks_from_user(user_id).await?;
    let missing_feedback: Vec<FeedbackListEntry> = missing_feedback
        .into_iter()
        .map(|(feedback, evaluation)| FeedbackListEntry {
            id: feedback.id,
            evaluation: FeedbackListEvaluationEnrty {
                team: evaluation.team,
                project: evaluation.project,
                begin_at: evaluation.begin_at,
            },
        })
        .collect();
    return Ok(HttpResponse::Ok().json(missing_feedback));
}

#[derive(Serialize)]
struct FeedbackInfo {
    pub id: i32,
    pub evaluation: EvaluationInfo,
    pub fields: Vec<FeedbackStructureField>,
}

#[derive(Serialize)]
struct EvaluationInfo {
    pub team: String,
    pub project: String,
    pub begin_at: chrono::DateTime<Utc>,
    pub correcteds: Vec<String>,
    pub corrector: String,
}

async fn evaluation_feedback_info(
    id: Identity,
    db: web::Data<Database>,
    feedback_id: web::Path<i32>,
    client: web::Data<awc::Client>,
    auth_client: web::Data<oauth2::basic::BasicClient>,
) -> Result<HttpResponse, ApiError> {
    let user_id: i32 = id.id().unwrap().parse::<i32>().unwrap();
    // log::warn!("DEBUG USER ID NO IDENTITY!");
    let feedback = db.get_evaluation_feedback(*feedback_id).await?;
    if user_id.ne(&feedback.user_id) {
        return Err(ApiError::Unauthorized);
    }
    let evaluation = db.get_evaluation(feedback.evaluation_id).await?;
    let scale_team = match get_scale_team(evaluation.scale_team_id, client, auth_client).await {
        Ok(scale_team) => scale_team,
        Err(_) => return Err(ApiError::InternalServerError),
    };
    let project = match scale_team.team.project_gitlab_path.rsplit_once("/") {
        Some(rem) => rem.1.to_owned(),
        None => scale_team.team.project_gitlab_path,
    };
    let team_names = scale_team
        .correcteds
        .iter()
        .map(|s| s.login.to_owned())
        .collect();
    let evaluation_info = EvaluationInfo {
        team: scale_team.team.name,
        project: project,
        begin_at: scale_team.begin_at,
        correcteds: team_names,
        corrector: scale_team.corrector.login,
    };
    let feedback_info = FeedbackInfo {
        id: *feedback_id,
        evaluation: evaluation_info,
        fields: FEEDBACK_EVALUATOR_FIELDS.to_vec(),
    };
    return Ok(HttpResponse::Ok().json(feedback_info));
}

// #[derive(Deserialize)]
// #[serde(untagged)]
// enum Feedback {
//     Evaluator(FeedbackEvaluator),
//     Evaluated(),
// }

async fn post_feedback(
    id: Identity,
    db: web::Data<Database>,
    feedback_id: web::Path<i32>,
    feedback_post: web::Json<FeedbackEvaluator>,
) -> Result<HttpResponse, ApiError> {
    let user_id: i32 = id.id().unwrap().parse::<i32>().unwrap();
    // log::warn!("DEBUG USER ID NO IDENTITY!");
    let mut feedback = db.get_evaluation_feedback(*feedback_id).await?;
    if user_id.ne(&feedback.user_id) | feedback.feedback.is_some() {
        return Err(ApiError::Unauthorized);
    }
    
    feedback.feedback = Some(serde_json::json!(feedback_post));
    db.update_evaluation_feedback(feedback).await?;
    Ok(HttpResponse::Ok().finish())
}
