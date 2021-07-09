use std::cmp::Ordering;

pub struct Agent;

/// PacketA represent what agents should play.
/// It has a sample and its sample rate plus a
/// number for the agent to know in which order to play the samples
#[derive(Debug, Copy, Clone)]
pub struct PacketA;

/// PacketA represent what agents should play.
/// It has a sample and its sample rate plus a
/// number for the agent to know in which order to play the samples
#[derive(Debug, Copy, Clone)]
pub struct PacketC;

/// PacketA represent what agents should play.
/// It has a sample and its sample rate plus a
/// number for the agent to know in which order to play the samples
#[derive(Debug, Copy, Clone)]
pub struct PacketB;

#[derive(Debug, Eq, Ord, Copy, Clone)]
pub struct Sample(pub u32);

impl PartialEq for Sample {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Sample {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else if self.0 > other.0 {
            Some(Ordering::Greater)
        } else if self.0 < other.0 {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

/// Send packet A to `agent`
///
pub fn send_packet_a(agent: &Agent, packet: PacketA) {
    dbg!("pkt a sent");
}

pub fn wait_for_sample() -> Sample {
    dbg!("wait sample");
    Sample(0)
}

pub fn wait_for_packet_b() -> PacketB {
    dbg!("wait missing packets feedback");
    PacketB {}
}

pub fn send_packet_c() -> PacketC {
    dbg!("wait missing packets feedback");
    PacketC {}
}
