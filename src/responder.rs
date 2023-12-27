use crate::common::node::Node;
use std::sync::{Arc, Mutex};


#[derive(Debug)]
pub struct Responder {
    
    node: Arc<Node>,

}

impl Responder {

    pub fn new(node: Arc<Node>) -> Self {
        Responder {
            node
        }
    }

    pub(crate) fn vote_election(&self,)
}