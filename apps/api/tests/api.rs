use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode, header},
};
use chrono::{Local, NaiveDate};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use std::sync::{Arc, RwLock};
use tempfile::NamedTempFile;
use tower::ServiceExt;

use api::habit::{Habit, HabitError, HabitStore};
use api::storage::Storage;

async fn setup_app(habits: Vec<Habit>) -> (Router, NamedTempFile) {
    let file = NamedTempFile::new().unwrap();
    let storage = Storage::new(file.path().to_path_buf());
    let store = HabitStore { habits };

    storage.save_habits(&store).unwrap();
    let shared_state = Arc::new(RwLock::new(storage));
    (api::app(shared_state), file)
}

#[tokio::test]
async fn test_get_habits_returns_habit_list() {
    let today: NaiveDate = Local::now().date_naive();

    let habits = vec![Habit {
        name: "exercise".to_string(),
        completions: vec![today],
    }];
    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(Request::get("/habits").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!([{"name": "exercise", "streak": 1}]));
}

#[tokio::test]
async fn test_get_habits_returns_empty_list() {
    let habits = vec![];
    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(Request::get("/habits").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!([]));
}

#[tokio::test]
async fn test_add_new_habit() {
    let today: NaiveDate = Local::now().date_naive();

    let habits = vec![Habit {
        name: "exercise".to_string(),
        completions: vec![today],
    }];
    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(
            Request::post("/habits")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({"name": "floss"})).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body, json!( {"name": "floss", "streak": 0}));
}

#[tokio::test]
async fn test_duplicate_habit_returns_conflict() {
    let today: NaiveDate = Local::now().date_naive();

    let habits = vec![Habit {
        name: "exercise".to_string(),
        completions: vec![today],
    }];
    let (app, _file) = setup_app(habits).await;
    let response = app
        .oneshot(
            Request::post("/habits")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({"name": "exercise"})).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        body,
        json!({"error": HabitError::DuplicateHabit("exercise".to_string()).to_string()})
    );
}

#[tokio::test]
async fn test_complete_habit() {
    let habits = vec![Habit {
        name: "exercise".to_string(),
        completions: vec![],
    }];
    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(
            Request::post("/habits/exercise/completions")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body, json!({"name": "exercise", "streak": 1}));
}

#[tokio::test]
async fn test_complete_non_existent_habit() {
    let habits = vec![];
    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(
            Request::post("/habits/exercise/completions")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        body,
        json!({"error": HabitError::HabitNotFound("exercise".to_string()).to_string()})
    );
}

#[tokio::test]
async fn test_complete_habit_twice_returns_error() {
    let today: NaiveDate = Local::now().date_naive();
    let habits = vec![Habit {
        name: "exercise".to_string(),
        completions: vec![today],
    }];

    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(
            Request::post("/habits/exercise/completions")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        body,
        json!({"error": HabitError::DuplicateCompletion("exercise".to_string()).to_string()})
    );
}

#[tokio::test]
async fn test_malformed_json_returns_error() {
    let (app, _file) = setup_app(vec![]).await;

    let response = app
        .oneshot(
            Request::post("/habits")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{not valid json}"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body, json!({"error": "invalid request body"}));
}

#[tokio::test]
async fn test_empty_habit_name_returns_error() {
    let (app, _file) = setup_app(vec![]).await;

    let response = app
        .oneshot(
            Request::post("/habits")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({"name": ""})).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        body,
        json!({"error": "error: name must not be empty or whitespace"})
    );
}

#[tokio::test]
async fn test_whitespace_habit_name_returns_error() {
    let (app, _file) = setup_app(vec![]).await;

    let response = app
        .oneshot(
            Request::post("/habits")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({"name": "\n"})).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        body,
        json!({"error": "error: name must not be empty or whitespace"})
    );
}
