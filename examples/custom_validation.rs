//! Showcases custom validators and garde contexts
//!
//! Run the example using
//!
//! ```sh
//! cargo run --example custom_validation
//! ```
use axum::{response::IntoResponse, routing::post, Json, Router, Server};
use axum_garde::WithValidation;
use garde::Validate;
use serde::{Deserialize, Serialize};

// Define your valid scheme
#[derive(Debug, Serialize, Deserialize, Validate)]
#[garde(context(PasswordContext))]
struct Person {
    #[garde(ascii, length(min = 3, max = 25))]
    username: String,
    #[garde(custom(password_validation))]
    password: String,
}

// Define your custom context
#[derive(Debug, Clone)]
struct PasswordContext {
    complexity: usize,
}

// Define your custom validation
fn password_validation(value: &str, context: &PasswordContext) -> garde::Result {
    if value.len() < context.complexity {
        return Err(garde::Error::new("password is not strong enough"));
    }
    Ok(())
}

async fn custom_validation(
    // Perform validation on the request payload
    WithValidation(_, _): WithValidation<Person, Json<Person>>,
) -> impl IntoResponse {
    "Validation suceeed!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/check_password", post(custom_validation))
        // Create the application state
        .with_state(PasswordContext { complexity: 10 });
    println!("See example: http://127.0.0.1:8080/check_password");
    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
