use actix_web::Scope;
use actix_web::{HttpResponse, Responder, web};
use actix_web::{get, post, patch, delete};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
  pub name: String,
  pub tags: Vec<String>,
  pub finished: bool,
  pub from: super::user::User,
}

#[derive(Serialize, Deserialize)]
pub struct TargetProjectInfo {
  pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateProjectInfo {
  pub name: String,
  pub tags: Vec<String>,
  pub from: super::user::TargetUserInfo,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveProjectInfo {
  pub from: super::user::TargetUserInfo,
  pub target: TargetProjectInfo,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectUpdater {
  pub name: String,
  pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateProjectInfo {
  pub from: super::user::TargetUserInfo,
  pub target: String,
  pub updater: ProjectUpdater,
}

#[get("/")]
pub async fn get(
  state: web::Data<super::AppState>
) -> impl Responder {
  let projects = (&state.projects).read().unwrap();
  
  HttpResponse::Ok().json((*projects).clone())
}

#[post("/create")]
pub async fn create(
  project_info: web::Json<CreateProjectInfo>,
  state: web::Data<super::AppState>
) -> impl Responder {
  let users = (&state.users).read().unwrap();
  let mut projects = (&state.projects).write().unwrap();

  let mut result: Option<&super::user::User> = None;
  for user in &(*users) {
    if user.email == project_info.from.email && user.pawd == project_info.from.pawd {
      result = Some(user);
    }
  }
  if let Some(from) = result {
    for project in &(*projects) {
      if project.name == project_info.name {
        return HttpResponse::BadRequest().body("Duplicate project name");
      }
    }
    (*projects).push(Project {
      name: project_info.name.clone(),
      tags: project_info.tags.clone(),
      finished: false,
      from: from.clone(),
    });

    return HttpResponse::Ok().json((*projects).get((*projects).len() - 1));
  } else {
    return HttpResponse::Unauthorized().body("Invalid user info");
  }
}

#[delete("/remove")]
pub async fn remove(
  remove_info: web::Json<RemoveProjectInfo>,
  state: web::Data<super::AppState>
) -> impl Responder {
  let users = (&state.users).read().unwrap();
  let mut projects = (&state.projects).write().unwrap();
  let mut result: Option<&super::user::User> = None;
  for user in &(*users) {
    if user.email == remove_info.from.email && user.pawd == remove_info.from.pawd {
      result = Some(user);
    }
  }
  if let Some(_) = result {
    let mut target_index: Option<usize> = None;
    for (index, project) in (*projects).iter().enumerate() {
      if project.name == remove_info.target.name {
        target_index = Some(index);
      }
    }
    if let Some(index) = target_index {
      (*projects).remove(index);

      return HttpResponse::Ok().body("Yeah, smart ass")
    } else {
      return HttpResponse::BadRequest().body("Cannot find specified project");
    }
  } else {
    return HttpResponse::Unauthorized().body("Invalid user email or pawd");
  }
}

#[patch("/update")]
pub async fn update(
  update_info: web::Json<UpdateProjectInfo>,
  state: web::Data<super::AppState>
) -> impl Responder {
  let users = (&state.users).read().unwrap();
  let mut projects = (&state.projects).write().unwrap();
  let mut result: Option<&super::user::User> = None;
  for user in &(*users) {
    if user.email == update_info.from.email && user.pawd ==  update_info.from.pawd {
      result = Some(user);
    }
  }
  if let Some(from) = result {
    for project in &mut (*projects) {
      if project.name == update_info.target {
        project.name = update_info.updater.name.clone();
        project.tags = update_info.updater.tags.clone();
        project.from = (*from).clone();

        return HttpResponse::Ok().json(project);
      }
    }
    return HttpResponse::BadRequest().body("Cannot find specified project");
  } else {
    return HttpResponse::Unauthorized().body("Invalid user email or pawd");
  }
}

pub fn setup() -> Scope {
  Scope::new("/project")
    .service(get)
    .service(create)
    .service(remove)
    .service(update)
}
