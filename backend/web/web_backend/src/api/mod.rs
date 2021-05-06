pub mod queue;
pub mod workspace;
use actix_web::web::Data;
pub type DbPool = Data<db_connector::DbPool>;
pub type UserProvider = Data<crate::user_provider::UserProvider>;
