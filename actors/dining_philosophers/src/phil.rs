use std::time::Duration;

use actix::{clock::sleep, prelude::*};
use rand::Rng;

use PhilState::*;

#[derive(Default)]
enum PhilState {
    #[default]
    InProcess,
    FinishedThinking,
    FinishedEating,
}

#[derive(Default)]
pub struct Phil {
    next: Option<Recipient<HandSticks>>,
    stick_count: usize,
    state: PhilState,
    id: usize,
}

impl Phil {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl Actor for Phil {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Chain(pub Addr<Phil>);

impl Handler<Chain> for Phil {
    type Result = ();

    fn handle(&mut self, Chain(addr): Chain, _: &mut Self::Context) -> Self::Result {
        self.next = Some(Recipient::from(addr))
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Think;

impl Handler<Think> for Phil {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, _: Think, _: &mut Self::Context) -> Self::Result {
        let wait = rand::thread_rng().gen_range(5..20);
        println!("[{id}] Thinking...", id = self.id);

        Box::pin(
            sleep(Duration::from_secs(wait))
                .into_actor(self)
                .map(move |_, phil, _| phil.state = FinishedThinking),
        )
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct Eat;

impl Handler<Eat> for Phil {
    type Result = actix::prelude::ResponseActFuture<Self, ()>;

    fn handle(&mut self, _: Eat, _: &mut Self::Context) -> Self::Result {
        let wait = rand::thread_rng().gen_range(5..10);
        println!("[{id}] Eating...", id = self.id);

        Box::pin(
            sleep(Duration::from_secs(wait))
                .into_actor(self)
                .map(move |_, phil, _| phil.state = FinishedEating),
        )
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct HandSticks(pub usize);

impl Handler<HandSticks> for Phil {
    type Result = ();

    fn handle(&mut self, mut sticks: HandSticks, ctx: &mut Self::Context) -> Self::Result {
        match self.state {
            FinishedEating => {
                sticks.0 += self.stick_count;
                self.stick_count = 0;

                self.state = InProcess;
                ctx.address().do_send(Think);
            }

            FinishedThinking if sticks.0 > 0 => {
                let need_count = 2 - self.stick_count;
                let grabbed = sticks.0.min(need_count);
                self.stick_count += grabbed;
                sticks.0 -= grabbed;

                if self.stick_count == 2 {
                    self.state = InProcess;
                    ctx.address().do_send(Eat);
                }
            }

            _ => {}
        }

        self.next.as_ref().inspect(|next| next.do_send(sticks));
    }
}
