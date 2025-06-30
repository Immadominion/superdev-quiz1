use serde::Serialize;

#[derive(Serialize)]
pub struct SuperdevApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T: Serialize> SuperdevApiResponse<T> {
    pub fn success_response(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error_response(error_msg: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error_msg),
        }
    }

    pub fn error_response_with_status(
        error_msg: String,
    ) -> warp::reply::WithStatus<warp::reply::Json> {
        let response = Self {
            success: false,
            data: None,
            error: Some(error_msg),
        };
        warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::BAD_REQUEST,
        )
    }
}

#[derive(Serialize)]
pub struct SuperdevKeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct AccountMetaData {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct CreateTokenResponseSuperdev {
    pub program_id: String,
    pub accounts: Vec<AccountMetaData>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct MintTokenResponseQuiz {
    pub program_id: String,
    pub accounts: Vec<AccountMetaData>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct SignMessageResponseSuperdev {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct VerifyMessageResponseQuiz {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct SendSolResponseQuiz {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct TokenAccountInfoSuperdev {
    pub pubkey: String,
    pub isSigner: bool,
}

#[derive(Serialize)]
pub struct SendTokenResponseSuperdev {
    pub program_id: String,
    pub accounts: Vec<TokenAccountInfoSuperdev>,
    pub instruction_data: String,
}
