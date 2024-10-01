#[macro_use]
extern crate tracing;

use std::sync::LazyLock;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};
use serde::{Deserialize, Serialize};

const ENV_VOCABULARY_APP_DSN: &str = "VOCABULARY_APP_DSN";

pub static VOCABULARY_APP_DSN: LazyLock<String> = LazyLock::new(|| {
    std::env::var(ENV_VOCABULARY_APP_DSN).expect("could not load VOCABULARY_APP_DSN env")
});

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    if let Err(err) = dotenv::dotenv() {
        warn!(error = err.to_string(), "processing dotenv file");
    }

    // TODO: This will be moved to lazy
    let client = Client::with_uri_str(&*VOCABULARY_APP_DSN)
        .await
        .expect("todo");

    let db = client.database("vocabulary-app");
    let users_coll: Collection<Document> = db.collection("users");

    // Query
    let my_user = users_coll
        .find_one(doc! {
            "name": "Víctor"
        })
        .await
        .unwrap();

    // // build our application with a route
    // let app = Router::new()
    //     // `GET /` goes to `root`
    //     .route("/", get(root))
    //     // `POST /users` goes to `create_user`
    //     .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}

// // basic handler that responds with a static string
// async fn root() -> &'static str {
//     &*VOCABULARY_APP_DSN
// }

// async fn create_user(
//     // this argument tells axum to parse the request body
//     // as JSON into a `CreateUser` type
//     Json(payload): Json<CreateUser>,
// ) -> (StatusCode, Json<User>) {
//     // insert your application logic here
//     let user = User {
//         id: 1337,
//         username: payload.username,
//     };

//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, Json(user))
// }

// // the input to our `create_user` handler
// #[derive(Deserialize)]
// struct CreateUser {
//     username: String,
// }

// // the output to our `create_user` handler
// #[derive(Serialize)]
// struct User {
//     id: u64,
//     username: String,
// }
