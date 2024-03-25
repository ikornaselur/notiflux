use crate::error::{NotifluxError, NotifluxErrorType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: u64,
    topic: String,
    scope: String,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Subscribe(String),
    Broadcast(String),
}

pub fn get_action(token: &str, public_key: &[u8]) -> Result<Action, NotifluxError> {
    let key = jsonwebtoken::DecodingKey::from_ec_pem(public_key).map_err(|_| NotifluxError {
        message: Some("Invalid public key".to_owned()),
        error_type: NotifluxErrorType::JWTError,
    })?;
    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::ES256);
    let token_data =
        jsonwebtoken::decode::<Claims>(token, &key, &validation).map_err(|_| NotifluxError {
            message: Some("Invalid token".to_owned()),
            error_type: NotifluxErrorType::JWTError,
        })?;

    let Claims {
        sub: _,
        exp: _,
        topic,
        scope,
    } = token_data.claims;

    if scope == "subscribe" {
        Ok(Action::Subscribe(topic))
    } else if scope == "broadcast" {
        Ok(Action::Broadcast(topic))
    } else {
        Err(NotifluxError {
            message: Some(format!("Invalid scope: {}", scope)),
            error_type: NotifluxErrorType::JWTError,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_action_subscribe() {
        let public_key = include_bytes!("../scripts/public_key.pem");

        let token = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJub3RpZmx1eCIsImV4cCI6NDg2NDk4ODU3NiwidG9waWMiOiJmb28iLCJzY29wZSI6InN1YnNjcmliZSJ9.1sZDe6V5ccJEALFeuHQe4R0D_t35t9c1s3QP3odFxIPxxdGXJOq2G8BgrMpqO3bu4n_q0GmnbFyY7LXVgLJbPw";

        let action = get_action(token, public_key).unwrap();

        assert_eq!(action, Action::Subscribe("foo".to_owned()));
    }

    #[test]
    fn test_get_action_broadcast() {
        let public_key = include_bytes!("../scripts/public_key.pem");

        let token = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJub3RpZmx1eCIsImV4cCI6NDg2NDk4ODU4OCwidG9waWMiOiJiYXIiLCJzY29wZSI6ImJyb2FkY2FzdCJ9.KEk-9_i6Z17P1cB2m4_pt_LJrvhg2X4OrYWoqBVgvA0AtmcKyCOZcwUQiuoZ8rFwjvj9_KiFWK5hE-bRRnfQsA";

        let action = get_action(token, public_key).unwrap();

        assert_eq!(action, Action::Broadcast("bar".to_owned()));
    }

    #[test]
    fn test_get_action_invalid_scope() {
        let public_key = include_bytes!("../scripts/public_key.pem");

        let token = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJub3RpZmx1eCIsImV4cCI6NDg2NDk4ODcxMywidG9waWMiOiJiYXIiLCJzY29wZSI6ImludmFsaWQifQ.rxmGLj6ykIRRUVaZMj4tzQ2Gf12yQdEdRBy_kVdesYTPCFVCVSP7G-o-JoRwcX1dAAwryt-b3nuwXTVGy_ge4w";

        let action = get_action(token, public_key);

        assert!(action.is_err());

        let err = action.unwrap_err();
        assert_eq!(err.error_type, NotifluxErrorType::JWTError);
        assert_eq!(err.message, Some("Invalid scope: invalid".to_owned()));
    }
}
