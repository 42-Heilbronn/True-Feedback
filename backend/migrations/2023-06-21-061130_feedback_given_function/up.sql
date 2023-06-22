-- Create a trigger function to update the `feedback_at` column
-- when the `feedback` column is updated
CREATE OR REPLACE FUNCTION feedback_given() RETURNS TRIGGER AS $$
BEGIN
    IF NEW.feedback IS DISTINCT FROM OLD.feedback THEN
        NEW.feedback_at := CURRENT_TIMESTAMP;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION manage_feedback_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER feedback_given BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE feedback_given()', _tbl);
END;
$$ LANGUAGE plpgsql;
