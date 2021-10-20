use std::cmp::Ordering;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
struct TaggedSampleBuffer;

impl TaggedSampleBuffer {
    fn push(&mut self, tagged_sample: TaggedSample) {}
    fn pop(&mut self) -> TaggedSample {
        TaggedSample
    }
    fn isEmpty(&self) -> bool {
        false
    }
}

/*
#[derive(Debug, Copy, Clone)]
pub struct Master {
    tagged_sample_buffer: TaggedSampleBuffer,
}
*/

struct TaggedSample;

fn get_data() -> TaggedSample {
    // milliseconds
    let interval = 20;
    thread::sleep(Duration::from_millis(interval));
    TaggedSample
}

fn send_to_agents(tagged_sample: TaggedSample) {}

fn start_server() {
    let tagged_sample_buffer = Arc::new(Mutex::new(TaggedSampleBuffer));
    let missing_samples = Arc::new(Mutex::new(TaggedSampleBuffer));

    let handle = {
        let tagged_sample_buffer = tagged_sample_buffer.clone();
        thread::spawn(move || loop {
            // TODO
            // Tag the sample here
            let tagged_sample = get_data();
            //
            match tagged_sample_buffer.lock() {
                Ok(mut value) => {
                    value.push(tagged_sample);
                }
                Err(error) => {
                    unimplemented!()
                }
            }
        })
    };

    let handle_2 = {
        let tagged_sample_buffer = tagged_sample_buffer.clone();
        thread::spawn(move || loop {
            match tagged_sample_buffer.lock() {
                Ok(mut value) => {
                    // --------
                    // should add a loop here. shouldn't send samples to agents 1 by 1
                    // need to know sample rate here
                    // send a fixed ammount of seconds that can be configured by a controller
                    if !value.isEmpty() {
                        send_to_agents(value.pop())
                    };
                    // --------
                }
                Err(error) => {
                    unimplemented!()
                }
            }
        })
    };

    match handle.join() {
        Ok(value) => {
            // unimplemented!();

        }
        Err(error) => {
            // unimplemented!();
        }
    }
    match handle_2.join() {
        Ok(value) => {
            unimplemented!()
        }
        Err(error) => {
            unimplemented!()
        }
    }
}
