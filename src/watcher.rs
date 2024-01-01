use crate::common::node::Node;
use std::sync::{Arc, Mutex};


pub struct Watcher {
    
    node: Arc<Node>,

}

impl Watcher {

    pub fn new(node: Arc<Node>) -> Self {
        Watcher {
            node
        }
    }

    pub fn vote_election(&self, term: u64, candidate_id:u32, last_log_term: u64, last_log_index: u64) -> (u64, bool){
        let node = self.node;
        if term < node.current_term{
            return (term, false)
        }
        if let Some(candidate) = node.voted_for {
            if candidate != candidate_id {
                return (term, false)
            }
        }
        let term_valid = self.node.is_entry_present(last_log_index, last_log_term);
        (term, term_valid)
    }
    
}