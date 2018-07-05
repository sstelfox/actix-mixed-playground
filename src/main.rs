extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate env_logger;
extern crate futures;

#[macro_use]
extern crate log;

use actix::prelude::*;
use actix_web::{http, middleware, server, App, Error, HttpRequest, HttpResponse};
use dotenv::dotenv;
use futures::Future;

mod supervised_actor;

fn index(_req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    let act = Arbiter::system_registry().get::<supervised_actor::SupervisedActor>();
    act.send(supervised_actor::DeathThreat)
        .from_err()
        .and_then(|resp| {
            Ok(HttpResponse::Ok()
                .body(resp.unwrap())
                .into())
        })
}

fn random_work(_req: HttpRequest) -> &'static str {
    let act = Arbiter::system_registry().get::<supervised_actor::SupervisedActor>();
    act.do_send(supervised_actor::RandomWork);
    "Did some random work\n"
}

fn unreliable_work(_req: HttpRequest) -> &'static str {
    let act = Arbiter::system_registry().get::<supervised_actor::SupervisedActor>();
    act.do_send(supervised_actor::UnreliableWork);
    "Did some random work\n"
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let sys = actix::System::new("playground");

    Arbiter::system_registry().init_actor(|_ctx| {
        supervised_actor::SupervisedActor::default()
    });

    server::new(|| {
        info!("Starting up web worker");

        App::new()
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(http::Method::GET).with_async(index))
            .resource("/random", |r| r.method(http::Method::GET).with(random_work))
            .resource("/unreliable", |r| r.method(http::Method::GET).with(unreliable_work))
    })
        .bind("127.0.0.1:8000")
        .unwrap()
        .start();

    info!("Starting up all system actors");
    sys.run();
    info!("Shutting down system actors");
}
