extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate serde_json;
extern crate json;
#[macro_use] extern crate serde_derive;

use actix_web::{
    error, http, http::{header}, middleware, middleware::cors::Cors, server, App, AsyncResponder, Error,
    HttpRequest, HttpResponse, Responder
};

use futures::{Future, Stream};

extern crate agon_rust;
use agon_rust::decode_document;
// use agon_rust::serving::request;

// mod model;
// use model::{ ClassifyResults };

// const ADDR: &str = "35.190.227.116:8500";

fn classify(req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    println!("model: {:?}", req);

    req.concat2()
        .from_err()
        .and_then(|body| {
            // println!("body   :{:?}", body);
            let (url, decoded_doc, is_get_all_text) = decode_document::decode(body).map_err(|e| {
                error::ErrorBadRequest(e)
            })?;
            // println!("{}", url);
            // println!("{}", decoded_doc);
            // println!("{}", is_get_all_text);
            if decoded_doc.len() == 0 {
                println!("decoded_doc length is 0");
            }

            // let result = request(ADDR, &decoded_doc).unwrap();
            // println!("{:?}", result);

            // let classify_results = ClassifyResults::dummy();
            
            Ok(HttpResponse::Ok()
                .content_type("plain/text")
                .header("X-Hdr", "sample")
                .body("true"))

            // Ok(HttpResponse::Ok()
            //     .json(classify_results))
            //     // .json(result))

            // match decode_document::decode(body) {
            //     Ok(decoded_doc) => {
            //         println!("{}", decoded_doc);
            //         let res_msg = String::from("ok ok ok");

            //         Ok(HttpResponse::Ok()
            //             .content_type("plain/text")
            //             .header("X-Hdr", "sample")
            //             .body(res_msg))
            //     },
            //     Err(err) => Err(error::ErrorBadRequest(err))
            // }
            
        })
       .responder()
}

fn hello(_req: HttpRequest) -> impl Responder {
    "Hello from the index page"
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("json-example");

    server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .configure(|app| {
                Cors::for_app(app)
                    // .allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::ORIGIN, header::CONTENT_TYPE, header::ACCEPT])
                    .allowed_header(header::ACCEPT)
                    .resource("/", |r| r.method(http::Method::GET).f(hello))
                    .resource("/classify", |r| r.method(http::Method::POST).f(classify))
                    .register()
            })
    }).bind("0.0.0.0:8080")
        .unwrap()
        .shutdown_timeout(1)
        .start();

    println!("Started http server: 0.0.0.0:8080");
    let _ = sys.run();
}
