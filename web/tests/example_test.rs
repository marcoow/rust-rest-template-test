use axum::{body::Body, http::Method};
use axum_on_rails::test::helpers::{request, teardown, TestContext};
use axum_on_rails_procs::test;
use std::collections::HashMap;

mod common;

#[test]
async fn test_hello(context: &TestContext) {
    let response = request(
        &context.app,
        "/example",
        HashMap::new(),
        Body::empty(),
        Method::GET,
    )
    .await;

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"<h1>Hello, World!</h1>");
}
