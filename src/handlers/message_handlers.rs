use base58::{FromBase58, ToBase58};
use base64::{Engine as _, engine::general_purpose};
use ed25519_dalek::{
    Signature as Ed25519Signature, Signer as Ed25519Signer, SigningKey, Verifier, VerifyingKey,
};
use crate::types::request_types::{SignMessageRequestQuiz, VerifyMessageRequestSuperdev};
use crate::types::response_types::{SuperdevApiResponse, SignMessageResponseSuperdev, VerifyMessageResponseQuiz};

// Sign message endpoint - basic implementation
pub async fn sign_message_quiz(
    request: SignMessageRequestQuiz,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Basic validation
    if request.message.is_empty() || request.secret.is_empty() {
        return Ok(warp::reply::json(&SuperdevApiResponse::<
            SignMessageResponseSuperdev,
        >::error_response(
            "Missing required fields".to_string()
        )));
    }

    // Decode private key - no proper error handling
    let private_key_bytes = match request.secret.from_base58() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Ok(warp::reply::json(&SuperdevApiResponse::<
                SignMessageResponseSuperdev,
            >::error_response(
                "Invalid private key format".to_string(),
            )));
        }
    };

    // Check length - hardcoded value
    if private_key_bytes.len() != 32 {
        return Ok(warp::reply::json(&SuperdevApiResponse::<
            SignMessageResponseSuperdev,
        >::error_response(
            "Invalid private key length".to_string(),
        )));
    }

    // Create Ed25519 keypair - inefficient copying
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&private_key_bytes);
    let signing_key = SigningKey::from_bytes(&key_bytes);

    // Get public key for response
    let verifying_key = signing_key.verifying_key();
    let public_key_bytes = verifying_key.to_bytes();
    let public_key_string = public_key_bytes.to_base58();

    // Sign the message
    let message_bytes = request.message.as_bytes();
    let signature = signing_key.sign(message_bytes);
    let signature_b64 = general_purpose::STANDARD.encode(signature.to_bytes());

    let response_data = SignMessageResponseSuperdev {
        signature: signature_b64,
        public_key: public_key_string,
        message: request.message,
    };

    let response = SuperdevApiResponse::success_response(response_data);
    Ok(warp::reply::json(&response))
}

// Verify message endpoint - more beginner style
pub async fn verify_message_superdev(
    request: VerifyMessageRequestSuperdev,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Decode public key
    let public_key_bytes = match request.pubkey.from_base58() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Ok(warp::reply::json(&SuperdevApiResponse::<
                VerifyMessageResponseQuiz,
            >::error_response(
                "Invalid public key format".to_string(),
            )));
        }
    };

    if public_key_bytes.len() != 32 {
        return Ok(warp::reply::json(&SuperdevApiResponse::<
            VerifyMessageResponseQuiz,
        >::error_response(
            "Invalid public key length".to_string(),
        )));
    }

    // Create public key - unsafe unwrap like a beginner
    let public_key = match VerifyingKey::from_bytes(&public_key_bytes.try_into().unwrap()) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(warp::reply::json(&SuperdevApiResponse::<
                VerifyMessageResponseQuiz,
            >::error_response(
                "Invalid public key".to_string()
            )));
        }
    };

    // Decode signature
    let signature_bytes = match general_purpose::STANDARD.decode(&request.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            return Ok(warp::reply::json(&SuperdevApiResponse::<
                VerifyMessageResponseQuiz,
            >::error_response(
                "Invalid signature format".to_string()
            )));
        }
    };

    if signature_bytes.len() != 64 {
        return Ok(warp::reply::json(&SuperdevApiResponse::<
            VerifyMessageResponseQuiz,
        >::error_response(
            "Invalid signature length".to_string()
        )));
    }

    let signature = Ed25519Signature::from_bytes(&signature_bytes.try_into().unwrap());

    // Verify signature
    let message_bytes = request.message.as_bytes();
    let is_valid = public_key.verify(message_bytes, &signature).is_ok();

    let response_data = VerifyMessageResponseQuiz {
        valid: is_valid,
        message: request.message,
        pubkey: request.pubkey,
    };

    let response = SuperdevApiResponse::success_response(response_data);
    Ok(warp::reply::json(&response))
}
