mod words;

use actix_files as fs;
use actix_web::{
    delete, get, patch, post, put, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
    Result,
};
use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}

#[get("/")]
async fn index() -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("message_from_rust", "hello from rust");
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/info")]
async fn info() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("info.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/words")]
async fn words_endpoint() -> impl Responder {
    let (word1, word2) = words::get_random_word_pair();
    let mut context = tera::Context::new();
    context.insert("word1", &word1);
    context.insert("word2", &word2);
    let page_content = TEMPLATES.render("words.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/word-pair")]
async fn word_pair_endpoint() -> impl Responder {
    let (word1, word2) = words::get_random_word_pair();
    let mut context = tera::Context::new();
    context.insert("word1", &word1);
    context.insert("word2", &word2);
    let page_content = TEMPLATES.render("word_pair.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

async fn favicon(_req: HttpRequest) -> Result<fs::NamedFile, actix_web::error::Error> {
    Ok(fs::NamedFile::open("image/favicon.ico")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/favicon.ico", web::get().to(favicon))
            .service(index)
            .service(info)
            .service(words_endpoint)
            .service(word_pair_endpoint)
            .service(fs::Files::new("/assets", "./assets").show_files_listing())
            .service(fs::Files::new("/image", "./image").show_files_listing())
    })
    .bind(("0.0.0.0", 10000))?
    .run()
    .await
}
