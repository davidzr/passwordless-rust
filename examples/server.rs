#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use passwordless_rust::{models::{RegisterRequest, SignInVerifyRequest}, PasswordlessClient};
use rocket::{fs::FileServer, serde::json::Json};
use rocket_dyn_templates::{context, Template};
use rocket::State;

use serde_json::{json, Value};

#[post("/register", format = "json", data = "<data>")]
pub async fn register(client: &State<PasswordlessClient>, data: Json<Value>) -> Value {
    let register_options = RegisterRequest {
        user_id: data.get("userId").unwrap().to_string(),
        username: data.get("username").unwrap().to_string(),
        display_name: data.get("username").unwrap().to_string(),
    };

    
    let token = client.register_token(&register_options).await.unwrap();
    json!(token)
}

#[post("/login", format = "json", data = "<data>")]
pub async fn login(client: &State<PasswordlessClient>, data: Json<Value>) -> Value {
    let request = SignInVerifyRequest {
        token: data.get("token").unwrap().to_string(),
    };
    let response = client.sign_in(&request).await.unwrap();
    json!(response)
}

#[get("/")]
pub fn index() -> Template {
    let passwordless_api_key =
        &std::env::var("PASSWORDLESS_API_KEY").expect("PASSWORDLESS_API_KEY must be set.");
    let passwordless_api_url =
        &std::env::var("PASSWORDLESS_API_URL").expect("PASSWORDLESS_API_URL must be set.");
    Template::render(
        "index",
        context! { passwordless_api_url: passwordless_api_url, passwordless_api_key: passwordless_api_key  },
    )
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let client = passwordless_rust::PasswordlessClient::new(
        &std::env::var("PASSWORDLESS_API_SECRET").expect("PASSWORDLESS_API_SECRET must be set."),
        &std::env::var("PASSWORDLESS_API_URL").expect("PASSWORDLESS_API_URL must be set."),
    );

    rocket::build()
        .mount("/", routes![index])
        .mount("/passwordless/api", routes![register, login])
        .mount("/static", FileServer::from("examples/static"))
        .manage(client)
        .attach(Template::fairing())
}
