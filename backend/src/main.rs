use tokio::net::TcpListener;
mod addons;
mod routes;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, routes::routes()).await.unwrap();
}
