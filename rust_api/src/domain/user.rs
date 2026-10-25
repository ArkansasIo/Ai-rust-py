use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole { User, Developer, Operator, Admin }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User { pub id: Uuid, pub username: String, pub role: UserRole, pub active: bool }
