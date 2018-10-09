use bcrypt::{verify};
use rand::prelude::*;
use rand::distributions::{Alphanumeric};

use actix_web::*;
use actix_web::middleware::session::{Session};

use models::{User, UserSession};

const USER_SESSION_KEY: &str  = "USER_SESSION";
const FLASH_MESSAGE_KEY: &str = "FLASH_MESSAGE";

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

pub fn set_flash_message(session: &Session, flash_message: FlashMessage) {
    session
        .set(FLASH_MESSAGE_KEY, flash_message)
        .expect("error set flash message");
}

pub fn get_flash_message(session: &Session) -> FlashMessage {
    let message = match session.get::<FlashMessage>(FLASH_MESSAGE_KEY) {
        Ok(Some(message)) => message,
        _                 => FlashMessage::new(),
    };
    set_flash_message(session, FlashMessage::new());

    message
}

pub fn is_signined(session: &Session) -> bool {
    match session.get::<UserSession>(USER_SESSION_KEY) {
        Ok(Some(_user_session)) => true,
        _                       => false,
    }
}

pub fn is_valid_password(password: &str, digest: &str) -> bool {
    verify(password, digest).unwrap()
}

pub fn is_valid_session_id(session: &Session, digest: &str) -> bool {
    match session.get::<UserSession>(USER_SESSION_KEY) {
        Ok(Some(user_session)) => verify(&user_session.session_id, digest).unwrap(),
        _                      => false,
    } 
}

//pub fn signin(user: &User, password: &str, session: &Session) -> Result<String, Error> {
pub fn signin(user: &User, password: &str, session: &Session) -> Result<UserSession, Error> {
    match verify(password, &user.password_digest.clone()) {
        Ok(true) => {
            let user_session = UserSession {
                user_id: user.id,
                session_id: create_session_id(),
            };

            session
                .set(USER_SESSION_KEY, user_session.clone())
                .expect("error set session id");

            Ok(user_session)
        },
        _ => {
            Err(error::ErrorForbidden("Forbidden"))
        }
    }
}

pub fn signout(session: &Session) {
    session
        .remove(USER_SESSION_KEY);
}

pub fn current_user_id(session: &Session) -> Option<i32> {
    match session.get::<UserSession>(USER_SESSION_KEY) {
        Ok(Some(user_session)) => Some(user_session.user_id),
        _                      => None,
    }
}

fn random_string(n: usize) -> String {
    use std::iter;

    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(n)
        .collect()
}

fn create_session_id() -> String {
    random_string(30)
}
