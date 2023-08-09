use crate::db::model::*;
use crate::db::schema::*;
use crate::db::Database;
use chrono::Utc;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;

impl Database {
    pub async fn get_evaluation(&self, id: i32) -> anyhow::Result<Evaluation> {
        let evaluation = evaluation::table
            .filter(evaluation::id.eq(id))
            .get_result(&mut self.pool.get().await?)
            .await?;
        Ok(evaluation)
    }

    pub async fn add_evaluation(&self, evaluation: NewEvaluation) -> anyhow::Result<Evaluation> {
        Ok(diesel::insert_into(evaluation::table)
            .values(evaluation)
            .get_result(&mut self.pool.get().await?)
            .await?)
    }

    pub async fn delete_evaluation(&self, scale_team_id: i32) -> anyhow::Result<Evaluation> {
        Ok(diesel::delete(evaluation::table)
            .filter(evaluation::scale_team_id.eq(scale_team_id))
            .get_result(&mut self.pool.get().await?)
            .await?)
    }

    pub async fn get_evaluation_feedback(&self, id: i32) -> anyhow::Result<EvaluationFeedback> {
        let feedback = evaluation_feedback::table
            .filter(evaluation_feedback::id.eq(id))
            .get_result(&mut self.pool.get().await?)
            .await?;
        Ok(feedback)
    }

    pub async fn add_evaluation_feedback(
        &self,
        feedback: NewEvaluationFeedback,
    ) -> anyhow::Result<EvaluationFeedback> {
        Ok(diesel::insert_into(evaluation_feedback::table)
            .values(feedback)
            .get_result(&mut self.pool.get().await?)
            .await?)
    }

    pub async fn update_evaluation_feedback(
        &self,
        feedback: EvaluationFeedback,
    ) -> anyhow::Result<EvaluationFeedback> {
        Ok(diesel::update(evaluation_feedback::table)
            .filter(evaluation_feedback::id.eq(feedback.id))
            .set(feedback)
            .get_result(&mut self.pool.get().await?)
            .await?)
    }

    pub async fn get_missing_evaluation_feedbacks_from_user(
        &self,
        user_id: i32,
    ) -> anyhow::Result<Vec<(EvaluationFeedback, Evaluation)>> {
        let feedback = evaluation_feedback::table
            .inner_join(evaluation::table)
            // .filter(evaluation::begin_at.le(Utc::now().naive_utc() - chrono::Duration::minutes(15)))
            .filter(evaluation_feedback::user_id.eq(user_id))
            .filter(evaluation_feedback::feedback.is_null())
            .get_results(&mut self.pool.get().await?)
            .await?;
        Ok(feedback)
    }
}
