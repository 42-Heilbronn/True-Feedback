CREATE TABLE evaluation (
    id SERIAL PRIMARY KEY,
    scale_team_id INT NOT NULL,
    team VARCHAR NOT NULL,
    project VARCHAR NOT NULL,
    evaluator_id INT NOT NULL,
    begin_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
