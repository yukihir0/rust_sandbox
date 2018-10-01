use bcrypt::{verify};

use actix_web::middleware::session::{Session};

const USER_SESSION_KEY: &str  = "USER_SESSION";
const FLASH_MESSAGE_KEY: &str = "FLASH_MESSAGE";

#[derive(Serialize, Deserialize)]
struct UserSession {
    user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct FlashMessage {
    pub error_messages: Vec<String>,
}

impl FlashMessage {
    pub fn new() -> Self {
        FlashMessage {
            error_messages: Vec::new(),
        }
    }
}

//pub fn set_flash_message(session: Session, message: String) {
pub fn set_flash_message(session: Session, flash_message: FlashMessage) {
    session
        .set(FLASH_MESSAGE_KEY, flash_message)
        .expect("error set flash message");
}

//pub fn get_flash_message(session: Session) -> String {
pub fn get_flash_message(session: Session) -> FlashMessage {
    let message = match session.get::<FlashMessage>(FLASH_MESSAGE_KEY) {
        Ok(Some(message)) => message,
        _                 => FlashMessage::new(),
    };
    set_flash_message(session, FlashMessage::new());

    message
}

pub fn is_signined(session: Session) -> bool {
    match session.get::<UserSession>(USER_SESSION_KEY) {
        Ok(Some(_user_session)) => true,
        _                       => false,
    }
}

pub fn is_valid_password(password: &str, digest: &str) -> bool {
    verify(password, digest).unwrap()
}

pub fn signin(session: Session, user_id: i32) {
    session
        .set(USER_SESSION_KEY, UserSession{user_id: user_id})
        .expect("error set session id");
}

pub fn signout(session: Session) {
    session
        .remove(USER_SESSION_KEY);
}

pub fn current_user_id(session: Session) -> Option<i32> {
    match session.get::<UserSession>(USER_SESSION_KEY) {
        Ok(Some(user_session)) => Some(user_session.user_id),
        _                      => None,
    }
}
