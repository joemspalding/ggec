use anyhow::{Result, Context};

// https://github.com/pond3r/ggpo/blob/master/src/lib/ggpo/game_input.h
// https://github.com/pond3r/ggpo/blob/master/src/lib/ggpo/game_input.cpp

pub trait GameInputTrait<I>
where I: PartialEq {
    fn new(input: I) -> Self;
    fn init() -> Self;
    fn equal(&self, other: &GameInput<I>, inputs_only: bool) -> bool;
}

/// T is your specific input format
#[derive(Clone, Copy)]
pub struct GameInput<I> 
where I: PartialEq {
    pub frame: Option<usize>,
    input: Option<I>
}



impl <I: PartialEq> GameInputTrait<I> for GameInput<I>{
    /// Used to initialize if we know the inputs coming in
    fn new(input: I) -> Self {
        Self {
            frame: None,
            input: Some(input)
        }
    }

    /// Used to initialize if we want an empty GameInput object
    fn init() -> Self {
        Self {
            frame: None,
            input: None
        }
    }

    // maybe return a result if I need more information next time
    fn equal(&self, other: &GameInput<I>, inputs_only: bool) -> bool {
        if !inputs_only && self.frame != other.frame {
            // maybe return a detailed error message here
            return false;
        }
        if self.input != other.input {
            // maybe return a detailed error message here
            return false;

        }
        true
    }
}
