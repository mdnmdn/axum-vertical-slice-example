use crate::features::todos::models::{CreateTodoPayload, Todo, UpdateTodoPayload};
use crate::tests::helpers::spawn_app;
use reqwest::StatusCode;

#[tokio::test]
async fn create_todo_returns_201_for_valid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let new_todo = CreateTodoPayload {
        title: "Test todo".to_string(),
    };

    let response = client
        .post(&format!("{}/todos", &app.address))
        .json(&new_todo)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::CREATED);

    let saved = response.json::<Todo>().await.unwrap();
    assert_eq!(saved.title, "Test todo");
    assert_eq!(saved.completed, false);
}

#[tokio::test]
async fn get_todos_returns_a_list_of_todos() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Create a todo to be retrieved
    let new_todo = CreateTodoPayload {
        title: "Test todo".to_string(),
    };
    client
        .post(&format!("{}/todos", &app.address))
        .json(&new_todo)
        .send()
        .await
        .expect("Failed to create todo.");

    let response = client
        .get(&format!("{}/todos", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let todos = response.json::<Vec<Todo>>().await.unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "Test todo");
}

#[tokio::test]
async fn get_todo_by_id_returns_correct_todo() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let new_todo = CreateTodoPayload {
        title: "Test todo".to_string(),
    };
    let response = client
        .post(&format!("{}/todos", &app.address))
        .json(&new_todo)
        .send()
        .await
        .expect("Failed to create todo.");
    let created_todo = response.json::<Todo>().await.unwrap();

    let response = client
        .get(&format!("{}/todos/{}", &app.address, created_todo.id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);
    let fetched_todo = response.json::<Todo>().await.unwrap();
    assert_eq!(fetched_todo.id, created_todo.id);
}

#[tokio::test]
async fn update_todo_returns_200_for_valid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let new_todo = CreateTodoPayload {
        title: "Test todo".to_string(),
    };
    let created_todo: Todo = client
        .post(&format!("{}/todos", &app.address))
        .json(&new_todo)
        .send()
        .await
        .expect("Failed to create todo.")
        .json()
        .await
        .unwrap();

    let update_data = UpdateTodoPayload {
        title: Some("Updated todo".to_string()),
        completed: Some(true),
    };

    let response = client
        .put(&format!("{}/todos/{}", &app.address, created_todo.id))
        .json(&update_data)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);

    let updated_todo = response.json::<Todo>().await.unwrap();
    assert_eq!(updated_todo.title, "Updated todo");
    assert_eq!(updated_todo.completed, true);
}

#[tokio::test]
async fn delete_todo_returns_204() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let new_todo = CreateTodoPayload {
        title: "Test todo".to_string(),
    };
    let created_todo: Todo = client
        .post(&format!("{}/todos", &app.address))
        .json(&new_todo)
        .send()
        .await
        .expect("Failed to create todo.")
        .json()
        .await
        .unwrap();

    let response = client
        .delete(&format!("{}/todos/{}", &app.address, created_todo.id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let get_response = client
        .get(&format!("{}/todos/{}", &app.address, created_todo.id))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}
