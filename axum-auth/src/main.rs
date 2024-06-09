use axum::{async_trait, extract::FromRequestParts, http::{header, request::Parts, StatusCode}, response::Response, routing::{get, post}, Json, Router};
use jwt_lib::User;
use serde_json::json;
use tokio::net::TcpListener;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let app = Router::new()
        .route("/public_view", get(public_view_handler))
        .route("/get_token", post(get_token_handler))
        .route("/secret_view", get(secret_view_handler));

    let tcp_listener = TcpListener::bind("127.0.0.1:8082").await.expect("address in use");
    println!("\n Server is listening on http://127.0.0.1:8082");
    axum::serve(tcp_listener, app).await.expect("Error serving app");
}

async fn public_view_handler() -> Response<String> {

    let response_string = json!(
        {
            "message": "Public view <- available to all users",
        }
    ).to_string();

    Response::builder()
        .status(StatusCode::OK)
        .header("x-foo", "custom header")
        .header(header::CONTENT_TYPE, "application/json")
        .body(response_string)
        .unwrap_or_default()
}   

async fn get_token_handler(Json(user): Json<User>) -> Response<String> {
    let token = jwt_lib::get_jwt(user);

    match token {
        Ok(token) => {
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(json!({
                    "success": true,
                    "token": token
                }).to_string())
                .unwrap_or_default()
        },
        Err(_) => {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(header::CONTENT_TYPE, "application/json")
                .body(json!({
                    "success": false,
                    "message": "Error generating token"
                }).to_string())
                .unwrap_or_default()
        }
    }
    
}

async fn secret_view_handler(Auth(user): Auth) -> Response<String> {
    Response::builder()
        .status(StatusCode::OK)
        .header("x-foo", "custom header")
        .header(header::CONTENT_TYPE, "application/json")
        .body(json!(
            {
                "message": "Secret view <- available to authorized users",
                "user": user
            }
        ).to_string())
        .unwrap_or_default()
}

struct Auth(User);

#[async_trait]
impl<S> FromRequestParts<S> for Auth where S: Send + Sync {
    type Rejection = Response<String>;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let access_token = parts.headers.get("Authorization");

        match access_token {
            Some(token) => {
                let token = token.to_str().unwrap();
                let token = token.replace("Bearer ", "");

                println!("Token: {:?}", token);

                let user = jwt_lib::decode_jwt(&token.to_string());

                println!("User: {:?}", user);

                match user {
                    Ok(user) => {
                        Ok(Auth(user))
                    },
                    Err(_) => {
                        let response_string = json!(
                            {
                                "message": "Unauthorized access",
                            }
                        ).to_string();

                        return Err(Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(response_string)
                            .unwrap_or_default()
                        )
                    }
                }
            },
            None => {
                let response_string = json!(
                    {
                        "message": "Unauthorized access",
                    }
                ).to_string();

                return Err(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(response_string)
                    .unwrap_or_default()
                )
            }
        }
    }
}