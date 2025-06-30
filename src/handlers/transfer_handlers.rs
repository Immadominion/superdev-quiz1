use base64::{Engine as _, engine::general_purpose};
use solana_sdk::pubkey::Pubkey;
use solana_system_interface::instruction as system_instruction;
use spl_token::instruction as token_instruction;
use crate::types::request_types::{SendSolRequestSuperdev, SendTokenRequestQuiz};
use crate::types::response_types::{SuperdevApiResponse, SendSolResponseQuiz, SendTokenResponseSuperdev, TokenAccountInfoSuperdev};

// Send SOL endpoint - basic implementation
pub async fn send_sol_quiz(
    request: SendSolRequestSuperdev,
) -> Result<impl warp::Reply, warp::Rejection> {
    let from = match request.from.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(warp::reply::json(
                &SuperdevApiResponse::<SendSolResponseQuiz>::error_response(
                    "Invalid from address".to_string(),
                ),
            ));
        }
    };

    let to = match request.to.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(warp::reply::json(
                &SuperdevApiResponse::<SendSolResponseQuiz>::error_response(
                    "Invalid to address".to_string(),
                ),
            ));
        }
    };

    // Basic validation - hardcoded check
    if request.lamports == 0 {
        return Ok(warp::reply::json(
            &SuperdevApiResponse::<SendSolResponseQuiz>::error_response(
                "Amount must be greater than 0".to_string(),
            ),
        ));
    }

    // Create transfer instruction
    let transfer_ix = system_instruction::transfer(&from, &to, request.lamports);

    // Convert to response format - simple approach
    let program_id = solana_sdk::system_program::id().to_string();
    let accounts_list = vec![from.to_string(), to.to_string()];
    let instruction_data = general_purpose::STANDARD.encode(&transfer_ix.data);

    let response_data = SendSolResponseQuiz {
        program_id,
        accounts: accounts_list,
        instruction_data,
    };

    let response = SuperdevApiResponse::success_response(response_data);
    Ok(warp::reply::json(&response))
}

// Send token endpoint - final endpoint with poor naming
pub async fn send_token_superdev(
    request: SendTokenRequestQuiz,
) -> Result<impl warp::Reply, warp::Rejection> {
    let destination = match request.destination.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(warp::reply::json(&SuperdevApiResponse::<
                SendTokenResponseSuperdev,
            >::error_response(
                "Invalid destination address".to_string(),
            )));
        }
    };

    let _mint = match request.mint.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(warp::reply::json(&SuperdevApiResponse::<
                SendTokenResponseSuperdev,
            >::error_response(
                "Invalid mint address".to_string()
            )));
        }
    };

    let owner = match request.owner.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(warp::reply::json(&SuperdevApiResponse::<
                SendTokenResponseSuperdev,
            >::error_response(
                "Invalid owner address".to_string()
            )));
        }
    };

    // Validation
    if request.amount == 0 {
        return Ok(warp::reply::json(&SuperdevApiResponse::<
            SendTokenResponseSuperdev,
        >::error_response(
            "Amount must be greater than 0".to_string(),
        )));
    }

    // Create a dummy token transfer instruction - beginner approach
    // In reality, we'd need source and destination token accounts
    let dummy_source = owner; // Using owner as source for simplicity
    let transfer_ix = token_instruction::transfer(
        &spl_token::id(),
        &dummy_source,
        &destination,
        &owner,
        &[],
        request.amount,
    )
    .unwrap();

    // Convert to response format
    let program_id = spl_token::id().to_string();
    let mut accounts_vec = Vec::new();

    // Simple conversion - not efficient
    for account in &transfer_ix.accounts {
        let account_info = TokenAccountInfoSuperdev {
            pubkey: account.pubkey.to_string(),
            isSigner: account.is_signer,
        };
        accounts_vec.push(account_info);
    }

    let instruction_data = general_purpose::STANDARD.encode(&transfer_ix.data);

    let response_data = SendTokenResponseSuperdev {
        program_id,
        accounts: accounts_vec,
        instruction_data,
    };

    let response = SuperdevApiResponse::success_response(response_data);
    Ok(warp::reply::json(&response))
}
