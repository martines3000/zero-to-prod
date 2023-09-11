use axum::http::{self, Request, StatusCode};
use hyper::Body;
use tower::ServiceExt;
use zero2prod::build_handler;

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = build_handler();

    // Create a request with form data
    let name = "Alice";
    let mail = "alice@gmail.com";
    let body = format!("name={}&email={}", name, mail); // <- form data

    // Send POST request with "application/x-www-form-urlencoded" body
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/subscriptions")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    // Status code should be 200 OK and body should be empty.
    assert_eq!(response.status(), StatusCode::OK);
    // TODO: Check that the subscription record is created in the database
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let test_cases = vec![
        (
            "name=Alice",
            "Input validation error: [email: Validation error: required [{\"value\": Null}]]",
        ),
        (
            "email=alice%40gmail.com",
            "Input validation error: [name: Validation error: required [{\"value\": Null}]]",
        ),
        ("", "Input validation error: [name: Validation error: required [{\"value\": Null}], email: Validation error: required [{\"value\": Null}]]"),
    ];

    for (body, error_msg) in test_cases {
        let app = build_handler();

        // Send POST request with "application/x-www-form-urlencoded" body
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/subscriptions")
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Status code should be 400 OK and body should contain the error_msg.
        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "The API did not fail with 400 Bad Request"
        );

        assert_eq!(
            std::str::from_utf8(&hyper::body::to_bytes(response.into_body()).await.unwrap())
                .unwrap(),
            error_msg,
        );
    }
}
