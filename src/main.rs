pub struct Player {}

pub trait State {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State>;
    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State>;
    fn print_self(&self, text: &str);
}

#[derive(Debug)]
pub struct StoppedState;

#[derive(Debug)]
pub struct PausedState;

#[derive(Debug)]
pub struct PlayingState;

impl dyn State {
    pub fn next(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        self.print_self("next");
        self
    }
    
    pub fn prev(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        self.print_self("prev");
        self
    }
}

impl State for StoppedState {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        self.print_self("play");
        Box::new(PlayingState)
    }

    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        self.print_self("stop");
        self
    }

    fn print_self(&self, text: &str) {
        println!("{self:?}::{text}");
    }
}

impl State for PausedState {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        self.print_self("play");
        Box::new(PlayingState)
    }

    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        self.print_self("stop");
        Box::new(StoppedState)
    }

    fn print_self(&self, text: &str) {
        println!("{self:?}::{text}");
    }
}

impl State for PlayingState {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        self.print_self("play");
        Box::new(PausedState)
    }

    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
        self.print_self("stop");
        Box::new(StoppedState)
    }

    fn print_self(&self, text: &str) {
        println!("{self:?}::{text}");
    }
}

fn main() {
    let mut player = Player {};
    let state = Box::new(StoppedState);
    let state = state.play(&mut player);
    let state = state.play(&mut player);
    let state = state.play(&mut player);
}
