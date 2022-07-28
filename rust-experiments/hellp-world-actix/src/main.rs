use actix_web::{
    get, guard, http::KeepAlive, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
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
#[allow(dead_code)]
async fn responder_req(_req: HttpRequest) -> impl Responder {
    web::Bytes::from_static(b"Hello world!")
}

// async fn index(req: HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {}

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
    HttpServer::new(move || {
        App::new()
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
    })
    // .bind_openssl("127.0.0.1:8080", builder)?
    .keep_alive(KeepAlive::Os)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
