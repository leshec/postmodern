mod words;

use axum::{
    http::{header, Response, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use lazy_static::lazy_static;
use tera::Tera;
use tower_http::services::ServeDir;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}

async fn index() -> Html<String> {
    let mut context = tera::Context::new();
    context.insert("message_from_rust", "hello from rust");
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    Html(page_content)
}

async fn info() -> Html<String> {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("info.html", &context).unwrap();
    Html(page_content)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn words_endpoint() -> Html<String> {
    let (word1, word2) = words::get_random_word_pair();
    let mut context = tera::Context::new();
    context.insert("word1", &word1);
    context.insert("word2", &word2);
    let page_content = TEMPLATES.render("words.html", &context).unwrap();
    Html(page_content)
}

async fn word_pair_endpoint() -> Html<String> {
    let (word1, word2) = words::get_random_word_pair();
    let mut context = tera::Context::new();
    context.insert("word1", &word1);
    context.insert("word2", &word2);
    let page_content = TEMPLATES.render("word_pair.html", &context).unwrap();
    Html(page_content)
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/info", get(info))
        .route("/word-pair", get(word_pair_endpoint))
        .route("/words", get(words_endpoint));

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handler_404);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
