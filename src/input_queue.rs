use anyhow::{Result, Context};
use crate::game_input::GameInput;



const INPUT_QUEUE_LENGTH: usize = 128;


pub trait InputQueueTrait<I>
where I: PartialEq + Clone {
    fn new(id: i32, input:I) -> Self;
    fn get_last_confirmed_frame(&self) -> Option<usize>;
    fn get_first_incorrect_frame(&self) -> Option<usize>;
    fn discard_confirmed_frames(&mut self, frame: usize);
    fn reset_prediction(&mut self, frame: usize);
    fn get_confirmed_input(&self, requested_frame: usize, input: &mut GameInput<I>) -> bool;
}

pub struct InputQueue<I>
where I: PartialEq + Clone {
    id: i32,
    head: usize,
    tail: usize,
    length: usize,
    frame_delay: usize,
    first_frame: usize,
    last_user_added_frame: Option<usize>,
    first_incorrect_frame: Option<usize>,
    last_frame_requested: Option<usize>,
    last_added_frame: Option<usize>,
    inputs: Vec<GameInput<I>>,
    prediction: GameInput<I>,
}


impl <I: PartialEq + Clone> InputQueueTrait<I> for InputQueue<I>{
    fn new(id: i32, input: I) -> Self {
        Self {
            id: id,
            head: 0,
            tail: 0,
            length: 0,
            frame_delay: 0,
            first_frame: 0,
            last_user_added_frame: None,
            first_incorrect_frame: None,
            last_frame_requested: None,
            last_added_frame: None,
            inputs: vec![GameInput::new(input)],
            prediction: GameInput::init()
        }
    }

    fn get_last_confirmed_frame(&self) -> Option<usize> {
        self.last_added_frame
    }

    fn get_first_incorrect_frame(&self) -> Option<usize> {
        self.first_incorrect_frame
    }

    fn discard_confirmed_frames(&mut self, frame: usize) {
        let frame = match self.last_frame_requested {
            Some(n) => std::cmp::min(frame, n),
            None => frame
        };

        if self.last_frame_requested == None
        || frame >= self.last_added_frame.unwrap() {
            self.tail = self.head;
        } else {
            let offset = frame - self.inputs[self.tail].frame.unwrap_or(0) as usize + 1;
            self.tail = self.tail % INPUT_QUEUE_LENGTH; //<- queue length

            self.length -= offset;
        }
    }

    fn reset_prediction(&mut self, frame: usize) {
        assert!(self.first_incorrect_frame == None || frame <= self.first_incorrect_frame.unwrap());
        
        self.prediction.frame = None;
        self.first_incorrect_frame = None;
        self.last_frame_requested = None;
    }

    fn get_confirmed_input(&self, requested_frame: usize, input: &mut GameInput<I>) -> bool {
        let offset: usize = requested_frame % INPUT_QUEUE_LENGTH;
        match self.inputs[offset].frame {
            Some(n) => {
                if n != requested_frame {
                    return false
                }
            },
            None => {}
        }
        *input = self.inputs[offset].clone();
        true
    }

    
}

#[cfg(test)]
mod input_queue {
    use super::InputQueue;

    #[test]
    fn can_make_new() {
        unimplemented!();
    }
}