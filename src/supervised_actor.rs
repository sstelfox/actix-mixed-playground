use actix::prelude::*;

#[derive(Message)]
pub struct DeathTest;

#[derive(Message)]
pub struct DoUnreliableWork;

#[derive(Message)]
pub struct RandomWork;

#[derive(Default)]
pub struct SupervisedActor;

impl Actor for SupervisedActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {
        info!("Supervised actor has been started");
    }

    fn stopping(&mut self, _: &mut Context<Self>) -> actix::Running {
        info!("Supervisor actor is about to stop");
        Running::Stop
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        info!("Supervisor actor was stopped");
    }
}

impl SystemService for SupervisedActor {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        info!("Starting up supervised actor as a service...");
    }
}

impl Handler<DeathTest> for SupervisedActor {
    type Result = ();

    fn handle(&mut self, _: DeathTest, ctx: &mut Context<Self>) {
        info!("Received death threat, committing supuku before they can get to me.");
        ctx.stop();
    }
}

impl Handler<RandomWork> for SupervisedActor {
    type Result = ();

    fn handle(&mut self, _: RandomWork, _ctx: &mut Context<Self>) {
        info!("Did some normal random work");
    }
}

impl actix::Supervised for SupervisedActor {
    fn restarting(&mut self, _ctx: &mut Context<Self>) {
        info!("SupervisedActor is restarting and stuff");
    }
}
