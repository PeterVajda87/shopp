use super::language::Language;
use uuid::Uuid;

pub struct User {
    user_id: Uuid,
    login_email: String,
    salt: String,
    password: String,
    language: Language,
}
