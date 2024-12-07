// use actix_cors::Cors;
use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{
    get, middleware::Logger, post, web, App, Error, HttpResponse, HttpServer, Responder,
};

use futures_util::stream::StreamExt as _;
use uuid::Uuid;

use std::fs::read;
use std::io::Write;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

#[get("/{route}")]
async fn index_routes() -> impl Responder {
    HttpResponse::Ok().body(read("./build/index.html").unwrap())
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(read("./build/index.html").unwrap())
}

#[post("/upload/")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    log::warn!("/upload/ was called!");
    while let Some(Ok(mut field)) = payload.next().await {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();

        match true {
            true => {}
            false => {}
        }

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        let filepath = format!("./uploads/{}", filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(Ok(chunk)) = field.next().await {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(HttpResponse::Found()
        .append_header(("Location", "/dashboard"))
        .finish())
}

#[get("/list/")]
async fn uploads(_: String) -> web::Json<Vec<(String, usize)>> {
    let mut files = vec![];
    for file in std::fs::read_dir("uploads/").unwrap() {
        let file = file.unwrap();
        #[cfg(windows)]
        let file_size = file.metadata().unwrap().file_size();
        #[cfg(unix)]
        let file_size = file.metadata().unwrap().size();
        files.push((
            file.file_name().to_str().unwrap().to_string(),
            (file_size / 1024) as usize,
        ))
    }
    web::Json(files)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8000");

    HttpServer::new(|| {
        App::new()
            // .wrap(Cors::permissive())
            .service(index)
            .service(index_routes)
            .service(Files::new("/static", "./build/static"))
            .service(upload)
            .service(uploads)
            .service(Files::new("/download/", "./uploads/"))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

use cookie::{Cookie, CookieJar};

fn use_cookie_without_secure() {
    let mut c = Cookie::new("name", "value");
    c.set_secure(true);
    c.set_secure(false);
}

fn use_cookie_builder_without_secure() {
    Cookie::build("name", "value").secure(true);
    Cookie::build("name", "value").secure(false);
}

fn use_cookie_without_http_only() {
    let mut c = Cookie::new("name", "value");
    c.set_http_only(true);
    c.set_http_only(false);
}

fn use_cookie_builder_without_http_only() {
    Cookie::build("name", "value").http_only(true);
    Cookie::build("name", "value").http_only(false);
}

fn header() {
    use http::header::{HeaderMap, SERVER, SET_COOKIE};
    let mut h = HeaderMap::new();
    h.insert(SERVER, 5.into());
    h.insert(SET_COOKIE, 42.into());
}

use parking_lot::{lock_api::RawMutex, Mutex, RwLock};

fn parking_lot() {
    let p_m: Mutex<()> = Mutex::const_new(0, ());
    let _ = p_m.lock();

    let p_m1 = Mutex::new(0);
    let _ = p_m1.lock();

    let p_rw = RwLock::new(0);
    let _ = p_rw.read();
    let _ = p_rw.write();
}
