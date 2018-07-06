use actix::prelude::*;
use futures::{future, Future};
use std::time::Duration;
use tokio_core::reactor::Timeout;

use super::supervised_actor;

#[test]
fn test_simple_handler() {
    // The simple handler shouldn't blow up, but it doesn't do anything so there really isn't
    // anything to "test". This will make sure that the simple call won't panic or take a huge
    // amount of time
    let system = System::new("test");

    supervised_actor::SupervisedActor::init_actor(|_| {
        supervised_actor::SupervisedActor::default()
    });

    Arbiter::handle().spawn_fn(move || {
        supervised_actor::SupervisedActor::from_registry()
            .send(supervised_actor::Simple)
            .then(move |something| {
                error!("Something: {:?}", something);

                Timeout::new(Duration::new(1, 0), Arbiter::handle())
                    .unwrap()
                    .then(move |_| {
                        Arbiter::system().do_send(actix::msgs::SystemExit(0));
                        future::result(Ok(()))
                    })
            })
    });

    system.run();
}
