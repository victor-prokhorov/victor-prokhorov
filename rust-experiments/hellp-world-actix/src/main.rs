use actix_web::{
    body::BoxBody,
    error, get, guard,
    http::{header::ContentType, KeepAlive},
    middleware::Logger,
    post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

use serde::{Deserialize, Serialize};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use futures::{future::ok, stream::once};
use std::sync::Mutex;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {app_name} from /app/index.html!")
}

struct AppState {
    app_name: String,
}

#[derive(Debug)]
struct AppStateWithCounter {
    counter: Mutex<i32>,
}

async fn mutable_state(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {counter}")
}

async fn print_without_increment(data: web::Data<AppStateWithCounter>) -> String {
    let counter = data.counter.lock().unwrap();
    format!("Request number: {counter}")
}

#[get("/openssl")]
async fn openssl_service(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

#[allow(dead_code)]
async fn simple_req(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

async fn responder_req(req: HttpRequest, data: web::Data<AppStateWithCounter>) -> impl Responder {
    log::info!("responder_req, req, {:?}", req);
    log::info!("responder_req, data, {:?}", data);
    let _bytes = web::Bytes::from_static(b"Hello world!");
    MyObj { name: "user" }
}

// async fn index(req: HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {}

#[get("/stream")]
async fn stream() -> HttpResponse {
    log::info!("stream");
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}
#[derive(Deserialize)]
struct User {
    name: String,
    id: u64,
}
#[derive(Deserialize)]
struct Qr {
    yes: String,
}
#[derive(Deserialize, Debug)]
struct JPayload {
    mega_field: u64,
}

// http://localhost:8080/users/666/fancyname?yes=yeeeees%20a%20lot
#[get("/users/{id}/{name}")]
async fn path_params_query(p: web::Path<User>, q: web::Query<Qr>) -> Result<String> {
    Ok(format!(
        "Hi {}, your id is {}, yes? {}",
        p.name, p.id, q.yes
    ))
}
#[post("/users")]
async fn post_users(j: web::Json<JPayload>) -> Result<String> {
    log::info!("json body {:?}", j);
    Ok(format!("json payload mega field {}", j.mega_field))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    // TODO:
    // try outside WSL
    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    // https://github.com/actix/examples/tree/master/https-tls/openssl
    // load TLS keys
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();

    // HttpServer automatically starts a number of HTTP workers, by default this number is equal to the number of logical CPUs in the system.
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    log::info!(target: "logging", "yay!");
    HttpServer::new(move || {
        let logger = Logger::default();
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new()
            .app_data(json_config)
            .wrap(logger)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(index))
                    .guard(guard::Header("x-guarded", "secret"))
                    .route(
                        "/guard",
                        web::to(|| async { HttpResponse::Ok().body("www") }),
                    ),
            )
            .app_data(counter.clone())
            .route("/counter", web::get().to(mutable_state))
            .route("/print-count", web::get().to(print_without_increment))
            .route("/responder", web::get().to(responder_req))
            .service(stream)
            .service(path_params_query)
            .service(post_users)
    })
    // .bind_openssl("127.0.0.1:8080", builder)?
    .keep_alive(KeepAlive::Os)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
