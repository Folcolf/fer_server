use axum::{handler::Handler, middleware, Router, Server};
use dotenv::dotenv;
use std::net::SocketAddr;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub mod auth;
pub mod contact;
pub mod schema;
pub mod user;
pub mod utils;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    dotenv().ok();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut app = Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }));

    app = user::controllers::controller(&app);
    app = auth::controllers::controller(&app);
    app = contact::controllers::controller(&app);
    app = app
        .fallback((|| utils::middleware::handler_404()).into_service())
        .layer(middleware::from_fn(
            utils::middleware::print_request_response,
        ));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn route(path_param: String) -> String {
    format!("/api{}", path_param)
}
