use actix::prelude::*;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum SupervisedActorError {
    RandomFailure,
    Stopped,
}

impl Error for SupervisedActorError {
    fn description(&self) -> &str {
        use self::SupervisedActorError::*;

        match *self {
            RandomFailure => "some random failure occurred",
            Stopped => "the actor was stopped",
        }
    }
}

impl fmt::Display for SupervisedActorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

pub struct DeathThreat;

impl Message for DeathThreat {
    type Result = Result<String, SupervisedActorError>;
}

#[derive(Message)]
pub struct RandomWork;

#[derive(Message)]
pub struct UnreliableWork;

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

impl Handler<DeathThreat> for SupervisedActor {
    type Result = Result<String, SupervisedActorError>;

    fn handle(&mut self, _: DeathThreat, ctx: &mut Context<Self>) -> Self::Result {
        info!("Received death threat, committing supuku before they can get to me.");
        ctx.stop();

        Ok(String::from("shutting down supervised actor after receiving a death threat\n"))
    }
}

impl Handler<RandomWork> for SupervisedActor {
    type Result = ();

    fn handle(&mut self, _: RandomWork, _ctx: &mut Context<Self>) {
        info!("Did some normal random work");
    }
}

impl Handler<UnreliableWork> for SupervisedActor {
    type Result = ();

    fn handle(&mut self, _: UnreliableWork, _ctx: &mut Context<Self>) {
        // TODO: Have some work that can fail and handle it appropriately in
        // the web handler
        info!("Did something that could have failed");
    }
}

impl actix::Supervised for SupervisedActor {
    fn restarting(&mut self, _ctx: &mut Context<Self>) {
        info!("SupervisedActor is restarting and stuff");
    }
}
