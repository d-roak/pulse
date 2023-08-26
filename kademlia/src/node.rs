use rand::RngCore;

use crate::utils;

// 160 bit IDs
pub type PeerId = [u8; 20];
pub type Key = [u8; 20];

#[derive(Debug, Clone)]
pub struct Peer {
    pub ip_address: String,
    pub udp_port: u16,
    pub node_id: PeerId,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: PeerId,
    // sorted by last seen
    // least-recently seen node at the head
    // most-recently seen note at the tail
    // for ea 0 <= i < 160 -> keeps list of peers with a distance between 2ˆi - 2ˆi+1
    // small values of i the kbucket is generally empty
    // large i values, list grow to size k (parameter, eg 20)
    pub kbuckets: Vec<Vec<Peer>>,
}

impl Node {
    pub fn new(seed: Option<[u8; 32]>) -> Node {
        let mut id = [0u8; 20];
        if let Some(seed) = seed {
            let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
            rng.fill_bytes(&mut id);
        } else {
            rand::thread_rng().fill_bytes(&mut id);
        }

        Node {
            id,
            kbuckets: vec![],
        }
    }

    pub fn kbucket_add_peer(&mut self, peer: Peer) {
        let _distance = utils::xor_distance(&self.id, &peer.node_id);
    }

    pub fn node_lookup(&self, _target: PeerId) -> Vec<Peer> {
        vec![]
    }
}
