// Import the necessary models
use crate::models::string_body::StringBody;

// Import the necessary axum API modules
use axum::{
    Router,
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

// Used to serialize in line
use serde_json::json;

// Use regex to validate the request body
use regex::Regex;

// The easiest way to implement a handler is to use async functions that return a type that implements IntoResponse
// This can be a tuple of (StatusCode, Json<T>) or any other type that implements IntoResponse
// In this case we are returning a tuple of (StatusCode, Json<T>)
async fn get_string(Path(key): Path<String>) -> impl IntoResponse {
    match key.as_str() {
        "hello" => {
            return (
                StatusCode::OK,
                Json(StringBody::new("hello".to_string(), "world".to_string())),
            )
                .into_response();
        }
        "damn" => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "SERVER IS DOWN"})),
            )
                .into_response();
        }
        _ => {
            return (StatusCode::NOT_FOUND, Json(json!({"error": "NOT FOUND"}))).into_response();
        }
    }
}

async fn set_string(Json(string_body): Json<StringBody>) -> impl IntoResponse {
    // Here you would typically save the string to a database or perform some action
    // For this example, we will just return the string back

    // TODO: Manage HTTP/1.1 415 Unsupported Media Type error
    // TODO: Manage HTTP/1.1 422 Unprocessable Entity error

    let key_pattern = Regex::new(r"^[a-zA-Z0-9:\-_]{3,}$").unwrap();
    let value_pattern = Regex::new(r"^.+$").unwrap();

    if !key_pattern.is_match(&string_body.key) || !value_pattern.is_match(&string_body.value) {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "Key or value is in invalid format"})),
        )
            .into_response();
    }

    (StatusCode::OK, Json(string_body)).into_response()
}

// The routes function is where we define the routes for this controller
// The routes function is called in the main.rs file to register the routes with the axum router
pub fn routes() -> Router {
    Router::new()
        .route("/get_string/{key}", get(move |key| get_string(key)))
        .route("/set_string", post(set_string))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::test_helpers::TestClient;

    #[tokio::test]
    async fn test_get_string_cases() {
        let app: axum::routing::Router = axum::routing::Router::new()
            .route("/get_string/{key}", get(move |key| get_string(key)));

        struct TestCase<'a> {
            key: &'a str,
            expected_status: StatusCode,
            expected_body: serde_json::Value,
        }

        let cases = vec![
            TestCase {
                key: "hello",
                expected_status: StatusCode::OK,
                expected_body: serde_json::json!({
                    "key": "hello",
                    "value": "world"
                }),
            },
            TestCase {
                key: "damn",
                expected_status: StatusCode::INTERNAL_SERVER_ERROR,
                expected_body: serde_json::json!({
                    "error": "SERVER IS DOWN"
                }),
            },
            TestCase {
                key: "other",
                expected_status: StatusCode::NOT_FOUND,
                expected_body: serde_json::json!({
                    "error": "NOT FOUND"
                }),
            },
        ];

        let client = TestClient::new(app);

        for case in cases {
            let response = client.get(&format!("/get_string/{}", case.key)).await;
            assert_eq!(
                response.status(),
                case.expected_status,
                "Status mismatch for key: {}",
                case.key
            );

            let body: serde_json::Value = response.json::<serde_json::Value>().await;
            assert_eq!(
                body, case.expected_body,
                "Body mismatch for key: {}",
                case.key
            );
        }
    }

    #[tokio::test]
    async fn test_set_string() {
        let app: axum::routing::Router =
            axum::routing::Router::new().route("/set_string", post(set_string));

        let client = TestClient::new(app);

        let test_cases = vec![
            StringBody {
                key: "k_0".to_string(),
                value: "v_0".to_string(),
            },
            StringBody {
                key: "k-0".to_string(),
                value: "v-0".to_string(),
            },
            StringBody {
                key: "k:0".to_string(),
                value: "v:0".to_string(),
            },
            StringBody {
                key: "key".to_string(),
                value: "value".to_string(),
            },
            StringBody {
                key: "test_set_string1".to_string(),
                value: "value1".to_string(),
            },
            StringBody {
                key: "test_set_string:1".to_string(),
                value: "value:1".to_string(),
            },
            StringBody {
                key: "set_string_test-1".to_string(),
                value: "value-1".to_string(),
            },
            StringBody {
                key: "set_string_test_1".to_string(),
                value: "value_1".to_string(),
            },
            StringBody {
                key: "set_string_test-value:aaa-bbb-000".to_string(),
                value: "value-uuid".to_string(),
            },
            StringBody {
                key: "set_string_test:integer".to_string(),
                value: "1000".to_string(),
            },
            StringBody {
                key: "set_string_test:integer:negative".to_string(),
                value: "-1000".to_string(),
            },
            StringBody {
                key: "set_string_test:float".to_string(),
                value: "1000.01".to_string(),
            },
            StringBody {
                key: "set_string_test:float-negative".to_string(),
                value: "-1000.01".to_string(),
            },
            StringBody {
                key: "set_string_test:bool".to_string(),
                value: "true".to_string(),
            },
            StringBody {
                key: "set_string_test:bool_false".to_string(),
                value: "false".to_string(),
            },
            StringBody {
                key: "set_string_test:html".to_string(),
                value: "<b>bold!</b>".to_string(),
            },
            StringBody {
                key: "123".to_string(),
                value: "123".to_string(),
            },
        ];

        for valid_input in test_cases {
            let response = client.post("/set_string").json(&valid_input).await;
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(response.json::<StringBody>().await, valid_input);
        }
    }

    #[tokio::test]
    async fn test_set_string_invalid_key() {
        let app: axum::routing::Router =
            axum::routing::Router::new().route("/set_string", post(set_string));

        let client = TestClient::new(app);

        let test_cases = vec![
            StringBody {
                key: "_".to_string(),
                value: "v_0".to_string(),
            },
            StringBody {
                key: "-".to_string(),
                value: "v-0".to_string(),
            },
            StringBody {
                key: ":".to_string(),
                value: "v:0".to_string(),
            },
            StringBody {
                key: "12".to_string(),
                value: "v:12".to_string(),
            },
        ];

        for invalid_input in test_cases {
            let response = client.post("/set_string").json(&invalid_input).await;
            assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
            assert_eq!(
                response.json::<serde_json::Value>().await,
                serde_json::json!({"error": "Key or value is in invalid format"})
            );
        }
    }
}
