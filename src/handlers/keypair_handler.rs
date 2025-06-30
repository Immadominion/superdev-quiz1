use base58::ToBase58;
use solana_sdk::{signature::Signer, signer::keypair::Keypair};
use crate::types::response_types::{SuperdevApiResponse, SuperdevKeypairResponse};

// Generate keypair endpoint - beginner style implementation
pub async fn generate_keypair_superdev() -> Result<impl warp::Reply, warp::Rejection> {
    let keypair = Keypair::new();
    let pubkey_string = keypair.pubkey().to_string();
    let secret_bytes = keypair.to_bytes();
    let secret_string = secret_bytes.to_base58();

    let response_data = SuperdevKeypairResponse {
        pubkey: pubkey_string,
        secret: secret_string,
    };

    let response = SuperdevApiResponse::success_response(response_data);
    Ok(warp::reply::json(&response))
}
