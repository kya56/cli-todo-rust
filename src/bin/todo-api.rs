use actix_web::{App, HttpServer, web};
use std::sync::{Arc, Mutex};
use todo::api;
use todo::shared::file;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todos = file::load_todos();
    let state = web::Data::new(Arc::new(Mutex::new(todos)));

    HttpServer::new(move || App::new().app_data(state.clone()).configure(api::routes))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
