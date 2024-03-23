use crate::error::{NotifluxError, NotifluxErrorType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: u64,
    channel: String,
    scope: String,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Join(String),
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
        channel,
        scope,
    } = token_data.claims;

    if scope == "join" {
        Ok(Action::Join(channel))
    } else if scope == "broadcast" {
        Ok(Action::Broadcast(channel))
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
    fn test_get_action_join() {
        let public_key = include_bytes!("../scripts/public_key.pem");

        let token = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJub3RpZmx1eCIsImV4cCI6NDg2NDc5NDkyMywiY2hhbm5lbCI6ImZvbyIsInNjb3BlIjoiam9pbiJ9.bG-QjVslDfpAAN9_BLH68F7CoW2vGnBoDkKV7y98xPQKlvFBId1WvdSlXYEM87yloKOE3L673_yz7mbpZXkCYQ";

        let action = get_action(token, public_key).unwrap();

        assert_eq!(action, Action::Join("foo".to_owned()));
    }

    #[test]
    fn test_get_action_broadcast() {
        let public_key = include_bytes!("../scripts/public_key.pem");

        let token = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJub3RpZmx1eCIsImV4cCI6NDg2NDc5NTA4MCwiY2hhbm5lbCI6ImJhciIsInNjb3BlIjoiYnJvYWRjYXN0In0.sUjM6cLvJbL7Ij1HLOYuTBwuqck8ArrPxJ2bP59einj3FRh_GEj9mYBassH8bCtLdkXS0PrUomQhpUl_cEaIvQ";

        let action = get_action(token, public_key).unwrap();

        assert_eq!(action, Action::Broadcast("bar".to_owned()));
    }

    #[test]
    fn test_get_action_invalid_scope() {
        let public_key = include_bytes!("../scripts/public_key.pem");

        let token = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJub3RpZmx1eCIsImV4cCI6NDg2NDc5NTEyMywiY2hhbm5lbCI6ImJhciIsInNjb3BlIjoiaW52YWxpZCJ9.q7_QUzYa8E1tMLpZdtr68OM83z-mJKcPI4CLr2HJR_NuoWN4ThqDZnM_rRkoEIlCRS9fj93LZI53LbZnQqC9CQ";

        let action = get_action(token, public_key);

        assert!(action.is_err());

        let err = action.unwrap_err();
        assert_eq!(err.error_type, NotifluxErrorType::JWTError);
        assert_eq!(err.message, Some("Invalid scope: invalid".to_owned()));
    }
}
