mod handlers;
mod routes;
mod types;

use routes::superdev_routes::setup_routes_superdev;
use warp::Filter;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    let cors_config = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

    let all_routes = setup_routes_superdev().with(cors_config);

    println!("Superdev Quiz Server running on http://0.0.0.0:{}", port);
    warp::serve(all_routes).run(([0, 0, 0, 0], port)).await;
}
