extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate env_logger;

#[macro_use]
extern crate log;

use actix::prelude::*;
use actix_web::{http, middleware, server, App, HttpRequest};
use dotenv::dotenv;

mod supervised_actor;

fn index(_req: HttpRequest) -> &'static str {
    let act = Arbiter::system_registry().get::<supervised_actor::SupervisedActor>();
    act.do_send(supervised_actor::DeathTest);
    "Basic response\n"
}

fn random_work(_req: HttpRequest) -> &'static str {
    let act = Arbiter::system_registry().get::<supervised_actor::SupervisedActor>();
    act.do_send(supervised_actor::RandomWork);
    "Did some random work\n"
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let sys = actix::System::new("playground");

    let _: Addr<Syn, _> = supervised_actor::SupervisedActor::start_default();

    server::new(|| {
        info!("Starting up web worker");

        App::new()
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(http::Method::GET).with(index))
            .resource("/random", |r| r.method(http::Method::GET).with(random_work))
    })
        .bind("127.0.0.1:8000")
        .unwrap()
        .start();

    info!("Starting up all system actors");
    sys.run();
    info!("Shutting down system actors");
}
