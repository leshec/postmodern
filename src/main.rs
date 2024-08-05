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

async fn favicon(_req: HttpRequest) -> Result<fs::NamedFile, actix_web::error::Error> {
    Ok(fs::NamedFile::open("images/favicon.ico")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/images", "images").show_files_listing())
            .route("/favicon.ico", web::get().to(favicon))
            .service(index)
            .service(info)
    })
    .bind(("127.0.0.1", 42069))?
    .run()
    .await
}
