use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
pub struct Sample;
#[derive(Debug, Copy, Clone)]
pub struct SampleRate;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct ID(u64);

#[derive(Debug, Copy, Clone)]
struct PacketA(ID, Sample, SampleRate);

#[derive(Debug)]
struct PacketB(Vec<ID>);

impl PartialOrd for PacketA {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for PacketA {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl PartialEq for PacketA {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for PacketA {}

#[derive(Debug, Copy, Clone)]
pub struct PacketAFactory {
    next_id: ID,
}

impl PacketAFactory {
    pub fn new() -> PacketAFactory {
        Self { next_id: ID(0) }
    }
    pub fn new_packet(&mut self, sample: Sample, sample_rate: SampleRate) -> PacketA {
        let packet = PacketA(self.next_id, sample, sample_rate);
        self.next_id = ID(self.next_id.0 + 1);
        packet
    }
}

impl Default for PacketAFactory {
    fn default() -> Self {
        Self::new()
    }
}
