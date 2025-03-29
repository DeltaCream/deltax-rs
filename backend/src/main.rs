// mod app;
use axum::{Router, routing::get_service};
// use tower::ServiceBuilder;
use std::net::SocketAddr;
use tower_http::{
    services::ServeDir,
    // cors::{CorsLayer, Any},
};
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use tower_http::{compression::CompressionLayer, decompression::RequestDecompressionLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let static_service = get_service(ServeDir::new("static"));

    // Serve static files from the "static" directory
    // let static_service = get_service(ServeDir::new("static")).handle_error(|error: std::io::Error| {
    //     (
    //         axum::http::StatusCode::INTERNAL_SERVER_ERROR,
    //         format!("Unhandled internal error: {}", error),
    //     )
    // });

    // let cors = CorsLayer::new()
    // .allow_origin(Any) // Replace `Any` with specific origins for production
    // .allow_methods(Any)
    // .allow_headers(Any);

    // Build the Axum app to create a router
    let app = Router::new()
        // .route("/api", get(app::api::api))
        // .layer(cors)
        // .route("/", axum::routing::get(|| async { "Hello, World!" }))
        // .layer(ServiceBuilder::new().layer(RequestDecompressionLayer::new()).layer(CompressionLayer::new()),)
        .fallback_service(static_service);
        // .fallback(fallback)
        // .nest_service("/", static_service);
        // .nest_service("/static", static_service);

    println!("Hello, world!");
    println!("Hi, world!");

    let host = [0, 0, 0, 0]; //allows external connections from host machine to all available interfaces
    //[127, 0, 0, 1]; for loopback address
    let port = 8080; //previously 3000
    let addr = SocketAddr::from((host, port));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // Start tracing
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();
    tracing_subscriber::fmt::init();
    tracing::event!(tracing::Level::INFO, "main");

    println!("Backend Server running at http://{}", addr);

    // Run the server
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route for {}", uri),
    )
}
