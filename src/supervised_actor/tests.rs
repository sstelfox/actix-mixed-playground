use actix::prelude::*;
use futures::Future;

use super::*;

#[test]
fn simple_handler() {
    // The simple handler shouldn't blow up, but it doesn't do anything so there really isn't
    // anything to "test". This will make sure that the simple call won't panic or take a huge
    // amount of time
    let system = System::new("test");
    SupervisedActor::init_actor(|_| { SupervisedActor::default() });

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

#[test]
fn actor_recovery() {
    // This test really doesn't cover much, it does make sure that the StopActor message can be
    // handled correctly and won't panic, but we rely on the actix systems tests to ensure that
    // this work is actually getting recovered. This will ensure that we can continue sending
    // messages to the supervised actor after it has shut itself down (and recovered).
    //
    // Ultimately this is kind of contrived as I can't really see a reason for us to intentionally
    // stop the actor and continue handling tests, but it's good to show how it can be done.
    let system = System::new("test");
    SupervisedActor::init_actor(|_| { SupervisedActor::default() });

    Arbiter::handle().spawn_fn(|| {
        SupervisedActor::from_registry()
            .do_send(StopActor);

        SupervisedActor::from_registry()
            .send(Simple)
            .then(|res| {
                assert_eq!(res.unwrap(), ());

                Arbiter::system().do_send(actix::msgs::SystemExit(0));
                Ok(())
            })
    });

    system.run();
}
