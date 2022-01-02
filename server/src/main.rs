pub mod user;
pub mod project;

use actix_web::{HttpServer, App, web};
use std::sync::RwLock;

pub struct AppState {
  users: RwLock<Vec<user::User>>,
  projects: RwLock<Vec<project::Project>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let users: Vec<user::User> = Vec::new();
  let projects: Vec<project::Project> = Vec::new();

  let app_state = web::Data::new(AppState {
    users: RwLock::new(users),
    projects: RwLock::new(projects),
  });

  HttpServer::new(move || {
    App::new()
      .app_data(app_state.clone())
      .service(user::setup())
      .service(project::setup())
  })
  .bind(("127.0.0.1", 3000))?
  .run()
  .await
}
