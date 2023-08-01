use crate::db::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::enums::FeedbackKind;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Queryable,
    Selectable,
    AsChangeset,
    Identifiable,
)]
#[diesel(table_name = evaluation)]
pub struct Evaluation {
    pub id: i32,
    pub scale_team_id: i32,
    pub team: String,
    pub project: String,
    pub evaluator_id: i32,
    pub begin_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Queryable, Selectable, AsChangeset, Insertable,
)]
#[diesel(table_name = evaluation)]
pub struct NewEvaluation {
    pub scale_team_id: i32,
    pub team: String,
    pub project: String,
    pub evaluator_id: i32,
    pub begin_at: NaiveDateTime,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Queryable,
    Selectable,
    AsChangeset,
    Identifiable,
)]
#[diesel(table_name = evaluation_feedback)]
pub struct EvaluationFeedback {
    pub id: i32,
    pub evaluation_id: i32,
    pub user_id: i32,
    pub kind: FeedbackKind,
    pub feedback_id: Option<i32>,
    pub feedback: Option<Value>,
    pub feedback_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Queryable, Selectable, AsChangeset, Insertable,
)]
#[diesel(table_name = evaluation_feedback)]
pub struct NewEvaluationFeedback {
    pub evaluation_id: i32,
    pub kind: FeedbackKind,
    pub user_id: i32,
}
