use super::state::AppState;
use crate::shared::file;
use crate::shared::todo::{Todo, TodoList};
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(reset)
        .service(list)
        .service(create)
        .service(update)
        .service(mark_done)
        .service(undo_done)
        .service(delete);
}

#[derive(serde::Deserialize)]
pub struct CreateTodo {
    title: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateTodo {
    pub title: String,
}

#[derive(serde::Serialize)]
pub struct TodoResponse {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ListMode {
    All,
    Todo,
    Done,
}

#[derive(serde::Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_mode")]
    pub mode: ListMode,
}

fn default_mode() -> ListMode {
    ListMode::All
}

#[get("/todos")]
async fn list(state: web::Data<AppState>, query: web::Query<ListQuery>) -> HttpResponse {
    let todos = state.lock().unwrap();

    let items: Vec<&Todo> = match query.mode {
        ListMode::All => todos.list().iter().collect(),
        ListMode::Todo => todos.todo().collect(),
        ListMode::Done => todos.done().collect(),
    };

    let response: Vec<TodoResponse> = items.iter().map(|t| TodoResponse::from(*t)).collect();

    HttpResponse::Ok().json(response)
}

#[post("/todos")]
async fn create(state: web::Data<AppState>, payload: web::Json<CreateTodo>) -> HttpResponse {
    let mut todos = state.lock().unwrap();
    let todo = todos.add(payload.title.clone());
    file::save_todos(&todos);
    HttpResponse::Created().json(todo)
}

#[put("/todos/{id}")]
async fn update(
    state: web::Data<AppState>,
    id: web::Path<u64>,
    body: web::Json<UpdateTodo>,
) -> impl Responder {
    let mut todos = state.lock().unwrap();

    match todos.update_title(*id, &body.title) {
        Ok(_) => {
            file::save_todos(&todos);
            HttpResponse::Ok().finish()
        }
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

#[post("/todos/{id}/mark-done")]
async fn mark_done(state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut todos = state.lock().unwrap();

    match todos.mark(*id, true) {
        Ok(_) => {
            file::save_todos(&todos);
            HttpResponse::Ok().finish()
        }
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

#[post("/todos/{id}/undo-done")]
async fn undo_done(state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut todos = state.lock().unwrap();

    match todos.mark(*id, false) {
        Ok(_) => {
            file::save_todos(&todos);
            HttpResponse::Ok().finish()
        }
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

#[delete("/todos/{id}")]
async fn delete(state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut todos = state.lock().unwrap();

    match todos.remove(*id) {
        Ok(_) => {
            file::save_todos(&todos);
            HttpResponse::NoContent().finish()
        }
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

#[delete("/todos/reset")]
async fn reset(state: web::Data<AppState>) -> impl Responder {
    let mut todos = state.lock().unwrap();
    *todos = TodoList::new();

    let _ = file::save_todos(&todos);

    HttpResponse::Ok().body("Reset")
}

impl From<&Todo> for TodoResponse {
    fn from(t: &Todo) -> Self {
        Self {
            id: t.id,
            title: t.title.clone(),
            done: t.done,
        }
    }
}
