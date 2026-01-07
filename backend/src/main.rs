mod routes;
mod handlers;
mod app;
mod models;
mod services;
mod config;


use tokio::net::TcpListener;

use app::create_app;


#[tokio::main]
async fn main() {
    use dotenvy::dotenv;
    dotenv().ok();

    // create the application
    let app = create_app();


    // run our app with hyper, listening globally on port 4000
    let listener = TcpListener::bind("0.0.0.0:4000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}


