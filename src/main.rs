mod token;

use crate::token::{Token};
use axum::extract::Path;
use axum::http::{StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:token/:tx", get(get_address));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_address(Path(param): Path<(String, String)>) -> Result<impl IntoResponse, StatusCode> {
    let url_result = Token::try_explorer_url(param.0.as_str());
    match url_result {
        Ok(url) => Ok(Response::builder()
            .status(StatusCode::FOUND)
            .header("Location", url.join(param.1.as_str()).unwrap().as_str())
            .body("".to_string())
            .unwrap()
            .into_response()),
        Err(_) => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(format!("Unsupported token: {}", param.0))
            .unwrap()
            .into_response()),
    }
}
