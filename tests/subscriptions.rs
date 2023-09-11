use axum::http::{self, Request, StatusCode};
use hyper::{client::connect, Body};
use sqlx::{Connection, PgConnection};
use tower::ServiceExt;
use zero2prod::{build_handler, configuration};

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = build_handler();
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

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

    // Check that the subscription record is created in the database
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, mail);
    assert_eq!(saved.name, name);
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let test_cases = vec![
        (
            "name=Alice",
            vec!["email: Validation error: required [{\"value\": Null}]"],
        ),
        (
            "email=alice%40gmail.com",
            vec!["name: Validation error: required [{\"value\": Null}]"],
        ),
        (
            "",
            vec![
                "name: Validation error: required [{\"value\": Null}]",
                "email: Validation error: required [{\"value\": Null}]",
            ],
        ),
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

        let response_body = &hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_body = std::str::from_utf8(response_body).unwrap();

        for err in error_msg {
            assert!(
                response_body.contains(err),
                "The API did not fail with {} when the payload was {}",
                err,
                body
            );
        }
    }
}
