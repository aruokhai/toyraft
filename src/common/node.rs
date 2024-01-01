use core::time;
use std::{time::Duration, error::Error};
use rand::{self, Rng};

use super::entry::Entry;

pub struct Node {
    pub status: NodeStatus,
    pub election_timeout: Duration,
    pub current_term: u64,
    pub log: Vec<Entry>,
    pub voted_for: Option<u32>
}

pub enum NodeStatus {
    Follower,
    Candidate, 
    Leader { next_index: u64, match_index: u64 },
}

impl Node {

    pub fn new(current_term: u64, election_bound: (Duration, Duration)) -> Result<Self, ()>  {
        let time_difference = (election_bound.1 - election_bound.0).as_secs();
        let mut random_time =  time_difference.checked_mul(rand::thread_rng().gen::<u64>());
        if let Some(rand_time) = random_time {
            let election_timeout_option = rand_time.checked_add(election_bound.0.as_secs());
            if let Some(election_timeout) = election_timeout_option {
                let node = Node {
                    status: NodeStatus::Follower,
                    election_timeout: Duration::from_secs(election_timeout),
                    current_term,
                    log: vec![],
                    voted_for: None
                };
                return Ok(node)
            }
            return Err(());
        }
        return  Err(());
    }

    pub fn is_entry_present(&self, entry_index: u64, entry_term: u64) -> bool {
        let log = self.log;
        for entry in log {
            if (entry.index == entry_index) && (entry.term == entry_term) {
                return true
            }
        }
        false
    }





}