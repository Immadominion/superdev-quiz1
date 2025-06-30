use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTokenRequestQuiz {
    pub mintAuthority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Deserialize)]
pub struct MintTokenRequestSuperdev {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Deserialize)]
pub struct SignMessageRequestQuiz {
    pub message: String,
    pub secret: String,
}

#[derive(Deserialize)]
pub struct VerifyMessageRequestSuperdev {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Deserialize)]
pub struct SendSolRequestSuperdev {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Deserialize)]
pub struct SendTokenRequestQuiz {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}