use rocket::serde::Serialize;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::DbErr;

#[derive(Serialize)]
pub struct ErrorMessage {
    pub message: String,
}

#[derive(Responder)]
pub enum ErrorResponder {
    #[response(status = 404)]
    NotFound(()),

    #[response(status = 500)]
    InternalError(Json<ErrorMessage>),

    #[response(status = 400)]
    BadRequest(Json<ErrorMessage>),
}

//tbh i dont even know why im returning json
impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> Self {
        Self::InternalError(Json(ErrorMessage {
            message: format!("Internal Error: {err}"),
        }))
        // match err {
        //     DbErr::RecordNotFound(_) => ErrorResponder::NotFound(()),
        //     _ => ErrorResponder::InternalError(Json(ErrorMessage {
        //         message: "server internal error".into(),
        //     })),
        // }
    }
}
