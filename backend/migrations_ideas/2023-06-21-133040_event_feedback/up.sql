CREATE TABLE event_feedback (
    id SERIAL PRIMARY KEY,
    event_user_id INT NOT NULL REFERENCES event_user(id),
    user_id INT NOT NULL,
    feedback_type INT NOT NULL,
    feedback JSONB NULL,
    feedback_at TIMESTAMP NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    -- updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- SELECT diesel_manage_updated_at('event');
SELECT manage_feedback_at('event_feedback');
