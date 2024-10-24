use spin_sdk::{
    http::{Request, Response, IntoResponse},
    http_component,
    key_value::Store,
    // sqlite::{Connection, Value},
    variables,
    llm
};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};


// Define a serializable User type
#[derive(Serialize, Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_api(req: Request) -> Result<impl IntoResponse>  {
// fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let token = variables::get("token")?;
    let api_uri = variables::get("api_uri")?;
    let version = variables::get("version")?;
    println!("token is {token} and api_uri is {api_uri} and version is {version}");

    // Open the default key-value store
    let store = Store::open_default()?;
    Store::open("default-key-store")?;

    // Create an instance of a User object and populate the values
    let user = User {
        fingerprint: "0x1234".to_owned(),
        location: "Brisbane".to_owned(),
    };
    // Store the User object using the "my_json" key
    store.set_json("my_json", &user)?;
    // Retrieve the user object from the key-value store, using the "my_json" key
    let retrieved_user: User = store.get_json("my_json")?.context("user not found")?;
    println!("the retrieved user: {:?}", retrieved_user);


    // let mut router = Router::new();
    // println!("the frigging router: {:?}", router.to_string());
    // router.get("/goodbye/:planet", api::goodbye_planet);
    // router.any_async("/*", api::echo_wildcard);
    // Ok(router.handle(req))

    let model = llm::InferencingModel::Llama2Chat;
    let inference = llm::infer(model, "tell me a joke about dogs")?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        // .body("Hello, shitty")
        .body(format!("{:?}", inference))
        .build())
}


// mod api {
//     use super::*;

//     // /goodbye/:planet
//     pub fn goodbye_planet(_req: Request, params: Params) -> Result<impl IntoResponse> {
//         let planet = params.get("planet").expect("PLANET");
//         Ok(Response::new(200, planet.to_string()))
//     }

//     // /*
//     pub async fn echo_wildcard(_req: Request, params: Params) -> Result<impl IntoResponse> {
//         let capture = params.wildcard().unwrap_or_default();
//         Ok(Response::new(200, capture.to_string()))
//     }
// }