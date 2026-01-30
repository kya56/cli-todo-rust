use actix_web::web;

pub mod state;
pub mod todos;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(todos::routes);
}
