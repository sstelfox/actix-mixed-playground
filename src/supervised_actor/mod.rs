use actix::prelude::*;
use std::fmt;
use std::error::Error;
use rand::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum SupervisedActorError {
    IntermittentFailure
}

impl Error for SupervisedActorError {
    fn description(&self) -> &str {
        use self::SupervisedActorError::*;

        match *self {
            IntermittentFailure => "there was some kind of intermittent failure",
        }
    }
}

impl fmt::Display for SupervisedActorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

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

impl actix::Supervised for SupervisedActor {
    fn restarting(&mut self, _ctx: &mut Context<Self>) {
        info!("SupervisedActor failed for some reason and is now being restarted");
    }
}

#[derive(Message)]
pub struct Simple;

impl Handler<Simple> for SupervisedActor {
    type Result = ();

    fn handle(&mut self, _: Simple, _ctx: &mut Context<Self>) {
        info!("Did something really basic");
    }
}

#[derive(Message)]
pub struct StopActor;

impl Handler<StopActor> for SupervisedActor {
    type Result = ();

    fn handle(&mut self, _: StopActor, ctx: &mut Context<Self>) {
        info!("Received a message to stop the actor!");
        ctx.stop();
    }
}

pub struct RandomWork;

impl Message for RandomWork {
    type Result = Result<u32, SupervisedActorError>;
}

impl Handler<RandomWork> for SupervisedActor {
    type Result = Result<u32, SupervisedActorError>;

    fn handle(&mut self, _: RandomWork, _ctx: &mut Context<Self>) -> Self::Result {
        info!("Generated a random number");
        Ok(thread_rng().gen::<u32>())
    }
}

pub struct UnreliableWork(pub bool);

impl Message for UnreliableWork {
    type Result = Result<String, SupervisedActorError>;
}

impl Handler<UnreliableWork> for SupervisedActor {
    type Result = Result<String, SupervisedActorError>;

    fn handle(&mut self, data: UnreliableWork, _ctx: &mut Context<Self>) -> Self::Result {
        info!("Attempting to do something unreliable");

        if data.0 {
            info!("Successfully did the thing!");
            Ok(String::from("We did it just fine!"))
        } else {
            error!("The thing failed!");
            Err(SupervisedActorError::IntermittentFailure)
        }
    }
}
