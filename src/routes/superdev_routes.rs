use warp::Filter;
use crate::handlers::keypair_handler::generate_keypair_superdev;
use crate::handlers::token_handlers::{create_token_quiz, mint_token_superdev};
use crate::handlers::message_handlers::{sign_message_quiz, verify_message_superdev};
use crate::handlers::transfer_handlers::{send_sol_quiz, send_token_superdev};

// Routes setup - inconsistent naming style
pub fn setup_routes_superdev() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let keypair_route_superdev = warp::path("keypair")
        .and(warp::post())
        .and_then(generate_keypair_superdev);

    let create_token_route_quiz = warp::path!("token" / "create")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create_token_quiz);

    let mint_token_route_superdev = warp::path!("token" / "mint")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(mint_token_superdev);

    let sign_message_route_quiz = warp::path!("message" / "sign")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(sign_message_quiz);

    let verify_message_route_superdev = warp::path!("message" / "verify")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(verify_message_superdev);

    let send_sol_route_quiz = warp::path!("send" / "sol")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(send_sol_quiz);

    let send_token_route_superdev = warp::path!("send" / "token")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(send_token_superdev);

    // Combine all routes - basic approach
    keypair_route_superdev
        .or(create_token_route_quiz)
        .or(mint_token_route_superdev)
        .or(sign_message_route_quiz)
        .or(verify_message_route_superdev)
        .or(send_sol_route_quiz)
        .or(send_token_route_superdev)
}