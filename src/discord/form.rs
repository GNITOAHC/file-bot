use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn send_file_form() -> impl IntoResponse {
    match File::open("assets/index.html").await {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents).await {
                Ok(_) => return Html(contents).into_response(),
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to read the HTML file",
                    )
                        .into_response()
                }
            }
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to open the HTML file",
            )
                .into_response()
        }
    }
}
