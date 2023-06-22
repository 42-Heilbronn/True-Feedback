// @generated automatically by Diesel CLI.

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
    evaluation_feedback (id) {
        id -> Int4,
        evaluation_id -> Int4,
        user_id -> Int4,
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
