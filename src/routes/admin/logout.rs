use crate::session_state::TypedSession;
use crate::utils::e500;
use actix_web::HttpResponse;
use actix_web::http::header::ContentType;
use actix_web_flash_messages::FlashMessage;

const SUCCESS_MESSAGE: &str = "You have successfully logged out.";

pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        Ok(HttpResponse::Unauthorized().finish())
    } else {
        session.log_out();
        FlashMessage::info(SUCCESS_MESSAGE).send();
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(serde_json::json!({"message": SUCCESS_MESSAGE})))
    }
}
