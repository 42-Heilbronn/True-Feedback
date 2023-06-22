CREATE TABLE evaluation_feedback (
    id SERIAL PRIMARY KEY,
    evaluation_id INT NOT NULL REFERENCES evaluation(id) ON DELETE CASCADE,
    user_id INT NOT NULL,
    feedback JSONB NULL,
    feedback_at TIMESTAMP NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    -- updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- SELECT diesel_manage_updated_at('evaluation');
SELECT manage_feedback_at('evaluation');
