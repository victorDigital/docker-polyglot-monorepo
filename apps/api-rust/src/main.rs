use warp::Filter;

#[tokio::main]
async fn main() {
    // Health check endpoint
    let health = warp::path("health")
        .map(|| warp::reply::json(&"OK"));

    // Hello World endpoint
    let hello = warp::path::end()
        .map(|| warp::reply::json(&"Hello from Rust API!"));

    let routes = health.or(hello);

    println!("🦀 Rust API starting on 0.0.0.0:8080");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;
}
