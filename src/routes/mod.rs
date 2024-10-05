use actix_web::Responder;

pub mod api;
pub mod index;
pub mod song;

pub type GenResponse = Result<impl Responder, crate::errors::GenError>;
