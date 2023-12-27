use either::Either;
use http::version::Version;
use std::net::SocketAddr;
use hyper::{service::make_service_fn, Server};
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

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;


#[derive(Default)]
pub struct MyPublicNodeServices {}

#[tonic::async_trait]
impl PublicNodeServices for MyPublicNodeServices {
    async fn append_entries(
        &self,
        request: Request<AppendEntriesRequest>,
    ) -> Result<Response<AppendEntriesResponse>, Status> {
        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }

    async fn request_vote(
        &self,
        request: Request<RequestVoteRequest>,
    ) -> Result<Response<RequestVoteResponse>, Status> {
        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

pub async fn serve(http_bind: SocketAddr, grpc_bind: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let services = MyPublicNodeServices::default();

    println!("GreeterServer listening on {}", addr);

    let tonic = PublicNodeServicesServer::new(services);
    let mut warp = warp::service(warp::path("hello").map(|| "hello, world!"));

    Server::bind(&addr)
        .serve(make_service_fn(move |_| {
            let mut tonic = tonic.clone();
            std::future::ready(Ok::<_, Infallible>(tower::service_fn(
                move |req: hyper::Request<hyper::Body>| match req.version() {
                    Version::HTTP_11 | Version::HTTP_10 => Either::Left({
                        let res = warp.call(req);
                        Box::pin(async move {
                            let res = res.await.map(|res| res.map(EitherBody::Left))?;
                            Ok::<_, Error>(res)
                        })
                    }),
                    Version::HTTP_2 => Either::Right({
                        let res = tonic.call(req);
                        Box::pin(async move {
                            let res = res.await.map(|res| res.map(EitherBody::Right))?;
                            Ok::<_, Error>(res)
                        })
                    }),
                    _ => unimplemented!(),
                },
            )))
        }))
        .await?;

    Ok(())
}

enum EitherBody<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> http_body::Body for EitherBody<A, B>
where
    A: http_body::Body + Send + Unpin,
    B: http_body::Body<Data = A::Data> + Send + Unpin,
    A::Error: Into<Error>,
    B::Error: Into<Error>,
{
    type Data = A::Data;
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    fn is_end_stream(&self) -> bool {
        match self {
            EitherBody::Left(b) => b.is_end_stream(),
            EitherBody::Right(b) => b.is_end_stream(),
        }
    }

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        match self.get_mut() {
            EitherBody::Left(b) => Pin::new(b).poll_data(cx).map(map_option_err),
            EitherBody::Right(b) => Pin::new(b).poll_data(cx).map(map_option_err),
        }
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        match self.get_mut() {
            EitherBody::Left(b) => Pin::new(b).poll_trailers(cx).map_err(Into::into),
            EitherBody::Right(b) => Pin::new(b).poll_trailers(cx).map_err(Into::into),
        }
    }
}

fn map_option_err<T, U: Into<Error>>(err: Option<Result<T, U>>) -> Option<Result<T, Error>> {
    err.map(|e| e.map_err(Into::into))
}