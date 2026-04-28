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

use api::error::AppError;
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

    let today_str = today.format("%Y-%m-%d").to_string();
    assert_eq!(body[0]["name"].as_str().unwrap(), "exercise");
    assert_eq!(body[0]["streak"].as_u64().unwrap(), 1);
    assert!(
        body[0]["completions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d.as_str().unwrap() == today_str)
    );
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

    assert!(body.as_array().unwrap().is_empty());
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

    assert_eq!(body["name"].as_str().unwrap(), "floss");
    assert_eq!(body["streak"].as_u64().unwrap(), 0);
    assert!(body["completions"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_delete_habit() {
    let habits = vec![Habit {
        name: "exercise".to_string(),
        completions: vec![],
    }];
    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(
            Request::delete("/habits/exercise")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_delete_habit_not_found() {
    let habits = vec![];
    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(
            Request::delete("/habits/exercise")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let expected_message = HabitError::HabitNotFound("exercise".to_string()).to_string();
    assert_eq!(body["message"].as_str().unwrap(), expected_message);
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

    let expected_message = HabitError::DuplicateHabit("exercise".to_string()).to_string();
    assert_eq!(body["message"].as_str().unwrap(), expected_message);
}

#[tokio::test]
async fn test_complete_habit_records_completion() {
    let today = Local::now().date_naive();
    let today_str = today.format("%Y-%m-%d").to_string();

    let habits = vec![Habit {
        name: "exercise".to_string(),
        completions: vec![],
    }];
    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(
            Request::post(format!("/habits/exercise/completions/{}", today_str))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["name"].as_str().unwrap(), "exercise");
    assert_eq!(body["streak"].as_u64().unwrap(), 1);
    assert!(
        body["completions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d.as_str().unwrap() == today_str)
    );
}

#[tokio::test]
async fn test_get_habits_returns_empty_completions() {
    let habits = vec![Habit {
        name: "exercise".to_string(),
        completions: vec![],
    }];
    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(Request::get("/habits").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body[0]["name"].as_str().unwrap(), "exercise");
    assert_eq!(body[0]["streak"].as_u64().unwrap(), 0);
    assert!(body[0]["completions"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_delete_completion_removes_completion() {
    let today = Local::now().date_naive();
    let today_str = today.format("%Y-%m-%d").to_string();

    let habits = vec![Habit {
        name: "exercise".to_string(),
        completions: vec![today],
    }];
    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(
            Request::delete(format!("/habits/exercise/completions/{}", today_str))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["name"].as_str().unwrap(), "exercise");
    assert_eq!(body["streak"].as_u64().unwrap(), 0);
    assert!(body["completions"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_delete_completion_not_found() {
    let today = Local::now().date_naive();
    let today_str = today.format("%Y-%m-%d").to_string();

    let habits = vec![Habit {
        name: "exercise".to_string(),
        completions: vec![],
    }];
    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(
            Request::delete(format!("/habits/exercise/completions/{}", today_str))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let expected_message = HabitError::CompletionNotFound(today_str).to_string();
    assert_eq!(body["message"].as_str().unwrap(), expected_message);
}

#[tokio::test]
async fn test_complete_habit_not_found() {
    let today = Local::now().date_naive();
    let today_str = today.format("%Y-%m-%d").to_string();
    let habits = vec![];
    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(
            Request::post(format!("/habits/exercise/completions/{}", today_str))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let expected_message = HabitError::HabitNotFound("exercise".to_string()).to_string();
    assert_eq!(body["message"].as_str().unwrap(), expected_message);
}

#[tokio::test]
async fn test_delete_completion_habit_not_found() {
    let today = Local::now().date_naive();
    let today_str = today.format("%Y-%m-%d").to_string();
    let habits = vec![];
    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(
            Request::delete(format!("/habits/exercise/completions/{}", today_str))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let expected_message = HabitError::HabitNotFound("exercise".to_string()).to_string();
    assert_eq!(body["message"].as_str().unwrap(), expected_message);
}

#[tokio::test]
async fn test_complete_habit_duplicate_returns_conflict() {
    let today: NaiveDate = Local::now().date_naive();
    let today_str = today.format("%Y-%m-%d").to_string();
    let habits = vec![Habit {
        name: "exercise".to_string(),
        completions: vec![today],
    }];

    let (app, _file) = setup_app(habits).await;

    let response = app
        .oneshot(
            Request::post(format!("/habits/exercise/completions/{}", today_str))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    let expected_message = HabitError::DuplicateCompletion("exercise".to_string()).to_string();
    assert_eq!(body["message"].as_str().unwrap(), expected_message);
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

    assert_eq!(body["message"].as_str().unwrap(), "invalid request body");
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

    let expected_message =
        AppError::InvalidInput("name must not be empty or whitespace".to_string()).to_string();
    assert_eq!(body["message"].as_str().unwrap(), expected_message);
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

    let expected_message =
        AppError::InvalidInput("name must not be empty or whitespace".to_string()).to_string();
    assert_eq!(body["message"].as_str().unwrap(), expected_message);
}
