use either::Either;
use http::version::Version;
use http;
use std::net::SocketAddr;
use std::sync::{Arc, Condvar, Mutex};
use hyper::{service::make_service_fn, Server};
use hyper::body::Body;
use std::convert::Infallible;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tonic::{Request, Response, Status};
use tower::Service;
use warp::Filter;

use crate::toy_raft::public_node_services_server::{PublicNodeServicesServer, PublicNodeServices};
use crate::toy_raft::{RequestVoteRequest, AppendEntriesRequest, AppendEntriesResponse, RequestVoteResponse};
use crate::watcher::Watcher;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;


pub struct ServerApi {
    watcher: Arc<Watcher>,
}

impl ServerApi {
    pub fn new(
        watcher: Arc<Watcher>,
        
    ) -> Self {
        Self {
            watcher,
        }
    }
}

#[tonic::async_trait]
impl PublicNodeServices for ServerApi {
    

    async fn request_vote(
        &self,
        request: Request<RequestVoteRequest>,
    ) -> Result<Response<RequestVoteResponse>, Status> {
        let RequestVoteRequest { last_log_index, last_log_term, term, candidate_id } = request.into_inner();
        let result = self.watcher.vote_election(term, candidate_id, last_log_term, last_log_index);
        let reply = RequestVoteResponse {
            term: result.0,
            vote_granted: result.1,
        };
        Ok(Response::new(reply))
    }
}



