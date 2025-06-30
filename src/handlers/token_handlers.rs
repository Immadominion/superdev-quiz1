use base64::{Engine as _, engine::general_purpose};
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction as token_instruction;
use crate::types::request_types::{CreateTokenRequestQuiz, MintTokenRequestSuperdev};
use crate::types::response_types::{SuperdevApiResponse, CreateTokenResponseSuperdev, MintTokenResponseQuiz, AccountMetaData};

// Create token endpoint - poor error handling style
pub async fn create_token_quiz(
    request: CreateTokenRequestQuiz,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Parse mint authority - basic validation
    let mint_authority = match request.mintAuthority.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(warp::reply::json(&SuperdevApiResponse::<
                CreateTokenResponseSuperdev,
            >::error_response(
                "Invalid mint authority address".to_string(),
            )));
        }
    };

    // Parse mint pubkey
    let mint_pubkey = match request.mint.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(warp::reply::json(&SuperdevApiResponse::<
                CreateTokenResponseSuperdev,
            >::error_response(
                "Invalid mint address".to_string()
            )));
        }
    };

    // Create initialize mint instruction - hardcoded values like a beginner
    let init_mint_ix = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &mint_authority,
        None,
        request.decimals,
    )
    .unwrap();

    // Convert instruction to the required format - inefficient way
    let program_id = spl_token::id().to_string();
    let mut accounts_vec = Vec::new();

    // Add accounts manually - beginner approach
    for account in &init_mint_ix.accounts {
        let account_meta = AccountMetaData {
            pubkey: account.pubkey.to_string(),
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        };
        accounts_vec.push(account_meta);
    }

    let instruction_data = general_purpose::STANDARD.encode(&init_mint_ix.data);

    let response_data = CreateTokenResponseSuperdev {
        program_id,
        accounts: accounts_vec,
        instruction_data,
    };

    let response = SuperdevApiResponse::success_response(response_data);
    Ok(warp::reply::json(&response))
}

// Mint token endpoint - more beginner style code
pub async fn mint_token_superdev(
    request: MintTokenRequestSuperdev,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mint = match request.mint.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(warp::reply::json(&SuperdevApiResponse::<
                MintTokenResponseQuiz,
            >::error_response(
                "Invalid mint address".to_string()
            )));
        }
    };

    let authority = match request.authority.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(warp::reply::json(&SuperdevApiResponse::<
                MintTokenResponseQuiz,
            >::error_response(
                "Invalid authority address".to_string(),
            )));
        }
    };

    let destination = match request.destination.parse::<Pubkey>() {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(warp::reply::json(&SuperdevApiResponse::<
                MintTokenResponseQuiz,
            >::error_response(
                "Invalid destination address".to_string(),
            )));
        }
    };

    // Create mint to instruction
    let mint_to_ix = token_instruction::mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[],
        request.amount,
    )
    .unwrap();

    // Convert to response format - repetitive code
    let program_id = spl_token::id().to_string();
    let mut accounts_vec = Vec::new();

    for account in &mint_to_ix.accounts {
        let account_meta = AccountMetaData {
            pubkey: account.pubkey.to_string(),
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        };
        accounts_vec.push(account_meta);
    }

    let instruction_data = general_purpose::STANDARD.encode(&mint_to_ix.data);

    let response_data = MintTokenResponseQuiz {
        program_id,
        accounts: accounts_vec,
        instruction_data,
    };

    let response = SuperdevApiResponse::success_response(response_data);
    Ok(warp::reply::json(&response))
}