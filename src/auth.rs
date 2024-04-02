use crate::error::{NotifluxError, NotifluxErrorType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: u64,
    topics: Vec<String>,
    scope: String,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Subscribe(Vec<String>),
    Broadcast(Vec<String>),
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
        topics,
        scope,
    } = token_data.claims;

    if scope == "subscribe" {
        Ok(Action::Subscribe(topics))
    } else if scope == "broadcast" {
        Ok(Action::Broadcast(topics))
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

        let token = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJub3RpZmx1eCIsImV4cCI6NDg2NTY3ODI1NSwidG9waWNzIjpbImZvbyJdLCJzY29wZSI6InN1YnNjcmliZSJ9.qUIcgWAUOjG9QUvifJoAuxjFY8kwoHI-h3XrX3a_sDm3NXin4WYZIHmUN_c5XkfpHCWjOefMWQ8IplIZaj0PeA";

        let action = get_action(token, public_key).unwrap();

        assert_eq!(action, Action::Subscribe(vec!["foo".to_owned()]));
    }

    #[test]
    fn test_get_action_broadcast() {
        let public_key = include_bytes!("../scripts/public_key.pem");

        let token = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJub3RpZmx1eCIsImV4cCI6NDg2NTY3ODMzMCwidG9waWNzIjpbImJhciJdLCJzY29wZSI6ImJyb2FkY2FzdCJ9.izDZtXaKXtUSaRPrCozPiy2nuHmdOH0djGAavhVxUszcUNAeD8_d2ndMDHNYZEs4w49cnZQTqCLsF13ksW2gzA";

        let action = get_action(token, public_key).unwrap();

        assert_eq!(action, Action::Broadcast(vec!["bar".to_owned()]));
    }

    #[test]
    fn test_get_action_invalid_scope() {
        let public_key = include_bytes!("../scripts/public_key.pem");

        let token = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJub3RpZmx1eCIsImV4cCI6NDg2NTY3ODM0OCwidG9waWNzIjpbImJhciJdLCJzY29wZSI6ImludmFsaWQifQ.X2Rjy1X0xojvIVIAimqqq3wPTy_Kv33BChzwx5wMQluhuvltzXTcfNug0bAGxmzRjogqcLKxJpqMauI_1oUt6Q";

        let action = get_action(token, public_key);

        assert!(action.is_err());

        let err = action.unwrap_err();
        assert_eq!(err.error_type, NotifluxErrorType::JWTError);
        assert_eq!(err.message, Some("Invalid scope: invalid".to_owned()));
    }
}
