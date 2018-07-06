extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate rand;

#[macro_use]
extern crate log;

use actix::prelude::*;
use actix_web::{http, middleware, server, App, Error, HttpRequest, HttpResponse};
use dotenv::dotenv;
use futures::Future;
use rand::prelude::*;

mod supervised_actor;

fn simple(_req: HttpRequest) -> &'static str {
    supervised_actor::SupervisedActor::from_registry()
        .do_send(supervised_actor::Simple);

    "Did something very basic\n"
}

fn stop(_req: HttpRequest) -> &'static str {
    supervised_actor::SupervisedActor::from_registry()
        .do_send(supervised_actor::StopActor);

    "Stopping the background worker\n"
}

fn random_work(_req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    supervised_actor::SupervisedActor::from_registry()
        .send(supervised_actor::RandomWork)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(num) => {
                    let msg = format!("Received random number: {}\n", num);
                    Ok(HttpResponse::Ok().body(msg).into())
                }
                Err(err) => {
                    let err_msg = format!("An error occurred: {}\n", err);
                    Ok(HttpResponse::InternalServerError().body(err_msg).into())
                }
            }
        })
}

fn unreliable_work(_req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    let success = thread_rng().gen();

    supervised_actor::SupervisedActor::from_registry()
        .send(supervised_actor::UnreliableWork(success))
        .from_err()
        .and_then(|res| {
            match res {
                Ok(status) => {
                    let msg = format!("Received message: {}\n", status);
                    Ok(HttpResponse::Ok().body(msg).into())
                }
                Err(err) => {
                    let err_msg = format!("An error occurred: {}\n", err);
                    Ok(HttpResponse::InternalServerError().body(err_msg).into())
                }
            }
        })
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let sys = actix::System::new("playground");

    supervised_actor::SupervisedActor::init_actor(|_| {
        // The actor can have a custom initialization defined here. Either way this will trigger
        // the actor to start up before something requests it. This 'warm up' period ensures
        // requests that will make use of this later have a flat performance profile.
        supervised_actor::SupervisedActor::default()
    });

    server::new(|| {
        info!("Starting up web worker");

        App::new()
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(http::Method::GET).with(simple))
            .resource("/stop", |r| r.method(http::Method::GET).with(stop))
            .resource("/random", |r| r.method(http::Method::GET).with_async(random_work))
            .resource("/unreliable", |r| r.method(http::Method::GET).with_async(unreliable_work))
    })
        .bind("127.0.0.1:8000")
        .unwrap()
        .start();

    info!("Starting up all system actors");
    sys.run();
    info!("Shutting down system actors");
}
