use actix_web::Scope;
use actix_web::{post, delete, patch, Responder, web, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
  pub name: String,
  pub email: String,
  pub pawd: String,
  pub is_logged_in: bool,
}

impl User {
  pub fn new(name: String, email: String, pawd: String) -> User {
    User {
      name,
      email,
      pawd,
      is_logged_in: true
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserInfo {
  pub name: String,
  pub email: String,
  pub pawd: String,
}

#[derive(Serialize, Deserialize)]
pub struct TargetUserInfo {
  pub email: String,
  pub pawd: String,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveUserInfo {
  pub target: TargetUserInfo,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserInfo {
  pub target: TargetUserInfo,
  pub updater: CreateUserInfo,
}

pub fn find_user(
  users: &mut Vec<User>,
  user_info: web::Json<TargetUserInfo>
) -> Option<&mut User> {
  for user in users {
    if user_info.email == user.email && user_info.pawd == user.pawd {
      return Some(user);
    }
  }

  None
}

#[post("/create")]
pub async fn create(
  user_info: web::Json<CreateUserInfo>,
  state: web::Data<super::AppState>
) -> impl Responder {
  let mut users = (&state.users).write().unwrap();
  for user in &(*users) {
    if user_info.name == user.name || user_info.email == user.email {
      return HttpResponse::BadRequest().body("Duplicate user name or email");
    }
  }
  (*users).push(
    User::new(
      user_info.name.clone(),
      user_info.email.clone(),
      user_info.pawd.clone()
    )
  );

  HttpResponse::Ok().json((*users).get((*users).len() - 1))
}

#[delete("/remove")]
pub async fn remove(
  user_info: web::Json<RemoveUserInfo>,
  state: web::Data<super::AppState>
) -> impl Responder {
  let mut users = (&state.users).write().unwrap();
  let mut user_index: Option<usize> = None;
  for (index, user) in users.iter().enumerate() {
    if user.email == user_info.target.email && user.pawd == user_info.target.pawd {
      user_index = Some(index);
    }
  }
  if let Some(index) = user_index {
    (*users).remove(index);

    return HttpResponse::Ok().body("Yeah, smart ass");
  } else {
    return HttpResponse::BadRequest().body("Cannot find specified user");
  }
}

#[patch("/update")]
pub async fn update(
  update_info: web::Json<UpdateUserInfo>,
  state: web::Data<super::AppState>
) -> impl Responder {
  let mut users = (&state.users).write().unwrap();
  for user in &mut (*users) {
    if user.email == update_info.target.email && user.pawd == update_info.target.pawd {
      user.name = update_info.updater.name.clone();
      user.email = update_info.updater.email.clone();
      user.pawd = update_info.updater.pawd.clone();

      return HttpResponse::Ok().json(user);
    }
  }

  HttpResponse::BadRequest().body("Cannot find specified user")
}

#[post("/login")]
pub async fn login(
  user_info: web::Json<TargetUserInfo>,
  state: web::Data<super::AppState>
) -> impl Responder {
  let mut users = (&state.users).write().unwrap();
  for user in &mut (*users) {
    if user.email == user_info.email && user.pawd == user_info.pawd {
      user.is_logged_in = true;

      return HttpResponse::Ok().json(user);
    }
  }

  HttpResponse::BadRequest().body("Cannot find specified user")
}

#[post("/logout")]
pub async fn logout(
  user_info: web::Json<TargetUserInfo>,
  state: web::Data<super::AppState>
) -> impl Responder {
  let mut users = (&state.users).write().unwrap();
  let result = find_user(&mut (*users), user_info);
  if let Some(mut user) = result {
    user.is_logged_in = false;

    return HttpResponse::Ok().json(user);
  } else {
    return HttpResponse::BadRequest().body("Cannot find specified user");
  }
}

pub fn setup() -> Scope {
  Scope::new("/user")
    .service(create)
    .service(remove)
    .service(update)
    .service(login)
    .service(logout)
}
