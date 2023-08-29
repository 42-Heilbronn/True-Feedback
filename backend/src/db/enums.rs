use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(DbEnum, Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[ExistingTypePath = "crate::db::schema::sql_types::FeedbackKind"]
pub enum FeedbackKind {
    Evaluator,
    Evaluated,
}
