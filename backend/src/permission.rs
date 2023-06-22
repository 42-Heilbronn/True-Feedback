use actix_session::SessionExt;
use actix_web::{dev::ServiceRequest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Permission {
    Staff,
}

pub async fn extract_permission(req: &ServiceRequest) -> Result<Vec<Permission>, actix_web::Error> {
    let session = &req.get_session();
    let permissions = session.get::<Vec<Permission>>("permission").unwrap_or(Some(vec![])).unwrap_or(vec![]);
    Ok(permissions)
}
