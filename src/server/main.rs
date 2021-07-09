mod lib;
use rocket::figment::value::Num;
/*
fn main() {
    let samples = {
        let heap: BinaryHeap<Sample> = BinaryHeap::new();
        Arc::new(Mutex::new(heap))
    };

    let set_samples_backtrack = {
        let map: HashMap<u32, Sample> = HashMap::new();
        Arc::new(Mutex::new(map))
    };

    let wait_for_resends = {
        let samples = samples.clone();
        let bt = set_samples_backtrack.clone();
        thread::spawn(move || loop {
            // get packet ("fake" as fuck for now XDDDD)
            let pk = { samples.lock().expect("wtf").pop() }.unwrap();
            // sending fake packet as packets are Samples cuz im dumb
            send_packet_a(&Agent {}, PacketA {});
            thread::sleep(Duration::from_micros(1));

            let packets_to_resend = {
                let set: HashSet<u32> = HashSet::new();
                Arc::new(Mutex::new(set))
            };
            let sample = wait_for_sample();
            // construct Packet A with sample (Sample = A for now)
            let p = sample;
            samples.lock().expect("wtf").push(p.clone());
            bt.lock().expect("wtf").insert(p.0, p);
        })
    };
    let wait_for_sample = {
        let samples = samples.clone();
        let bt = set_samples_backtrack.clone();
        thread::spawn(move || loop {
            let sample = wait_for_sample();
            // construct Packet A with sample (Sample = A for now)
            let p = sample;
            samples.lock().expect("wtf").push(p.clone());
            bt.lock().expect("wtf").insert(p.0, p);
        })
    };


    let packet_a_continuous = {
        let samples = samples.clone();
        let bt = set_samples_backtrack.clone();
        thread::spawn(move || loop {
            // get packet ("fake" as fuck for now XDDDD)
            let pk = { samples.lock().expect("wtf").pop() }.unwrap();
            // sending fake packet as packets are Samples cuz im dumb
            send_packet_a(&Agent {}, PacketA {});
            thread::sleep(Duration::from_micros(1));
        })
    };

    wait_for_sample.join();
    packet_a_continuous.join();
}
*/

use rocket::futures::future::join;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::RwLock;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
struct Sample;
#[derive(Debug, Copy, Clone)]
struct SampleRate;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Numbering(u64);

#[derive(Debug, Copy, Clone)]
struct PacketA(Sample, Numbering, SampleRate);

#[derive(Debug)]
struct PacketB(Vec<Numbering>);

impl PartialOrd for PacketA {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl Ord for PacketA {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.1.cmp(&other.1) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl PartialEq for PacketA {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl Eq for PacketA {}

fn main() {
    /*
    let samples = {
        let heap: BinaryHeap<Sample> = BinaryHeap::new();
        Arc::new(Mutex::new(heap))
    };
    */

    let packet_a_backtrack: Arc<RwLock<HashMap<Numbering, PacketA>>> =
        Arc::new(RwLock::new(HashMap::new()));

    let packet_a_queue: Arc<RwLock<BinaryHeap<PacketA>>> = Arc::new(RwLock::new(BinaryHeap::new()));

    let wait_for_packet_b = {
        let packet_a_backtrack = packet_a_backtrack.clone();
        let packet_a_queue = packet_a_queue.clone();
        thread::spawn(move || loop {
            let packet = wait_for_packet_b();
            let packet_a_backtrack = packet_a_backtrack.read().expect("wtf");
            let mut packet_a_queue = packet_a_queue.write().expect("wtf");
            for n in packet.0 {
                match packet_a_backtrack.get(&n) {
                    Some(p) => {
                        packet_a_queue.push(*p);
                    }
                    None => {
                        println!("shouldn't happen");
                    }
                }
            }
        })
    };

    let wait_for_packet_a = {
        let packet_a_queue = packet_a_queue.clone();
        thread::spawn(move || loop {
            let packet = wait_for_packet_a();
            packet_a_queue.write().expect("fuck").push(packet);
        })
    };

    let send_packet_a = {
        let packet_a_backtrack = packet_a_backtrack.clone();
        let packet_a_queue = packet_a_queue.clone();
        thread::spawn(move || loop {
            let packet = packet_a_queue.write().expect("fuck").pop();
            match packet {
                Some(packet) => {
                    send_packet_a(packet);
                    packet_a_backtrack
                        .write()
                        .expect("ewsndsa")
                        .insert(packet.1, packet);
                }
                None => {
                    println!("shouldn't happen"); //maybe should, idk
                }
            };
        })
    };
}

fn wait_for_packet_b() -> PacketB {
    // fake network delay
    thread::sleep(Duration::from_nanos(1));
    PacketB(Vec::new())
}

fn wait_for_packet_a() -> PacketA {
    // fake network delay
    thread::sleep(Duration::from_nanos(1));
    PacketA(Sample {}, Numbering(0), SampleRate {})
}

fn send_packet_a(packet: PacketA) {}

mod packet_a_factory;