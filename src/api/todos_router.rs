use crate::application::todos::commands::complete_todo::__path_complete_todo;
use crate::application::todos::commands::delete_todo::__path_delete_todo;
use crate::application::todos::commands::update_todo::__path_update_todo;
use crate::application::todos::commands::create_todo::__path_create_todo;
use crate::application::todos::models::todo_list_model::TodoListModel;
use crate::application::todos::models::todo_details_model::TodoDetailsModel;
use crate::application::todos::models::create_todo_model::CreateTodoModel;
use crate::application::todos::models::update_todo_model::UpdateTodoModel;
use crate::application::todos::queries::get_todos::__path_get_todos;
use crate::application::todos::queries::get_todo::__path_get_todo;
use std::sync::Arc;
use axum::{Router};
use axum::routing::{get, patch, post, put};
use utoipa::OpenApi;


use crate::application::todos::commands::complete_todo::complete_todo;
use crate::application::todos::commands::create_todo::create_todo;
use crate::application::todos::commands::delete_todo::delete_todo;
use crate::application::todos::commands::update_todo::update_todo;
use crate::application::todos::queries::get_todo::get_todo;
use crate::application::todos::queries::get_todos::get_todos;


use crate::server::state::AppState;

pub fn todo_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/todos", get(get_todos).post(create_todo))
        .route("/api/todos/:id",
            get(get_todo)
            .put(update_todo)
            .delete(delete_todo))
        .route("/api/todos/:id/complete", patch(complete_todo))
        .with_state(app_state.clone())
}

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "todos_router", description = "Todo items management API")
    ),
    paths(get_todo, get_todos, create_todo, update_todo, delete_todo, complete_todo),
    components(schemas(CreateTodoModel, TodoDetailsModel, TodoListModel, UpdateTodoModel))
)]
pub(super) struct TodoApi;