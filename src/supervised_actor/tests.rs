use actix::prelude::*;
use futures::Future;

use super::*;

#[test]
fn simple_handler() {
    // The simple handler shouldn't blow up, but it doesn't do anything so there really isn't
    // anything to "test". This will make sure that the simple call won't panic or take a huge
    // amount of time
    let system = System::new("test");

    SupervisedActor::init_actor(|_| {
        SupervisedActor::default()
    });

    Arbiter::handle().spawn_fn(move || {
        SupervisedActor::from_registry()
            .send(Simple)
            .then(move |res| {
                assert_eq!(res.unwrap(), ());

                Arbiter::system().do_send(actix::msgs::SystemExit(0));
                Ok(())
            })
    });

    system.run();
}
