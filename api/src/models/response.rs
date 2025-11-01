use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// Custom serde implementation for reqwest::StatusCode
mod serde_status_code {
    use reqwest::StatusCode;
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serializer;
    use serde::de;

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<StatusCode, D::Error> {
        StatusCode::from_u16(u16::deserialize(deserializer)?).map_err(de::Error::custom)
    }

    pub fn serialize<S: Serializer>(status: &StatusCode, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u16(status.as_u16())
    }
}

/// Base for all API responses.
/// Success responses have a status code of 200, a message, and data model.
/// Error responses have a status code of 400 or greater, a message, and an error model.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<TData = ()> {
    #[serde(with = "serde_status_code")]
    pub status: StatusCode,
    pub message: String,
    pub data: Option<TData>,
    pub error: Option<ErrorResponse>,
}

impl<TData> Response<TData> {
    pub fn is_success(&self) -> bool {
        self.status >= StatusCode::OK && self.status < StatusCode::BAD_REQUEST
    }

    pub fn is_error(&self) -> bool {
        self.error.is_some() || self.status >= StatusCode::BAD_REQUEST
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ErrorType {
    #[serde(rename = "Server Error")]
    ServerError,
    #[serde(rename = "Database Error")]
    DatabaseError,
    #[serde(rename = "iMessage Error")]
    IMessageError,
    #[serde(rename = "Socket Error")]
    SocketError,
    #[serde(rename = "Validation Error")]
    ValidationError,
    #[serde(rename = "Authentication Error")]
    AuthenticationError,
    #[serde(rename = "Gateway Timeout")]
    GatewayTimeout,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub error_type: ErrorType,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_known_error_response() {
        let json = r#"{
            "status": 401,
            "message": "You are not authorized to access this resource",
            "error": {
                "type": "Authentication Error",
                "message": "Unauthorized"
            }
        }"#;
        let response: Response<String> = serde_json::from_str(json).unwrap();

        assert_eq!(
            response.status,
            StatusCode::UNAUTHORIZED,
            "Status code should be 401"
        );
        assert_eq!(
            response.message, "You are not authorized to access this resource",
            "Message should be \"You are not authorized to access this resource\""
        );
        let error = response.error.as_ref().expect("Error should be present");
        assert_eq!(
            error.error_type,
            ErrorType::AuthenticationError,
            "Error type should be ErrorType::AuthenticationError"
        );
        assert_eq!(
            error.message, "Unauthorized",
            "Error message should be \"Unauthorized\""
        );
    }

    #[test]
    fn parse_unknown_error_response() {
        let json = r#"{
            "status": 418,
            "message": "I'm a teapot",
            "error": {
                "type": "Teapot Error",
                "message": "I'm a teapot"
            }
        }"#;
        let response: Response<String> = serde_json::from_str(json).unwrap();

        assert_eq!(
            response.status,
            StatusCode::IM_A_TEAPOT,
            "Status code should be 418"
        );
        assert_eq!(
            response.message, "I'm a teapot",
            "Message should be \"I'm a teapot\""
        );
        let error = response.error.as_ref().expect("Error should be present");
        assert_eq!(
            error.error_type,
            ErrorType::Other("Teapot Error".to_string()),
            "Error type should be ErrorType::Other(\"Teapot Error\")"
        );
        assert_eq!(
            error.message, "I'm a teapot",
            "Error message should be \"I'm a teapot\""
        );
    }

    #[test]
    fn parse_success_response() {
        let json = r#"{
            "status": 200,
            "message": "Ping received!",
            "data": "pong"
        }"#;
        let response: Response<String> = serde_json::from_str(json).unwrap();

        assert_eq!(response.status, StatusCode::OK, "Status code should be 200");
        assert_eq!(
            response.message, "Ping received!",
            "Message should be \"Ping received!\""
        );
        let data = response.data.as_ref().expect("Data should be present");
        assert_eq!(data, "pong", "Data should be \"pong\"");
    }
}
