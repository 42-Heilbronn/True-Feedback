CREATE TABLE evaluation (
    id SERIAL PRIMARY KEY,
    scale_team_id INT NOT NULL,
    evaluator_id INT NOT NULL,
    begin_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
