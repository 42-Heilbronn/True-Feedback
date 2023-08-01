// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "feedback_kind"))]
    pub struct FeedbackKind;
}

diesel::table! {
    evaluation (id) {
        id -> Int4,
        scale_team_id -> Int4,
        team -> Varchar,
        project -> Varchar,
        evaluator_id -> Int4,
        begin_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FeedbackKind;

    evaluation_feedback (id) {
        id -> Int4,
        evaluation_id -> Int4,
        user_id -> Int4,
        kind -> FeedbackKind,
        feedback_id -> Nullable<Int4>,
        feedback -> Nullable<Jsonb>,
        feedback_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    event_feedback (id) {
        id -> Int4,
        event_user_id -> Int4,
        user_id -> Int4,
        feedback_type -> Int4,
        feedback -> Nullable<Jsonb>,
        feedback_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    event_user (id) {
        id -> Int4,
        event_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::joinable!(evaluation_feedback -> evaluation (evaluation_id));
diesel::joinable!(event_feedback -> event_user (event_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    evaluation,
    evaluation_feedback,
    event_feedback,
    event_user,
);
