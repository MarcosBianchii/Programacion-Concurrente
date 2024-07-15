use crate::phil::*;
use actix::prelude::*;

pub struct Table {
    phils: Vec<Addr<Phil>>,
}

impl Table {
    pub fn new(size: usize) -> Self {
        let phils: Vec<_> = (0..size).map(|id| Phil::new(id).start()).collect();

        // Initialize each philosopher with his next one.
        phils.iter().rev().fold(phils[0].clone(), |next, phil| {
            phil.do_send(Chain(next));
            phil.clone()
        });

        Self { phils }
    }

    pub fn start(&self) {
        // Make them think.
        self.phils.iter().for_each(|phil| {
            phil.do_send(Think);
        });

        // Hand the first one the sticks.
        self.phils[0].do_send(HandSticks(5));
    }
}
