
/*
 _        _______  _______  _       __________________ _______
| \    /\(  ___  )(  ____ \( (    /|\__   __/\__   __/(  ___  )
|  \  / /| (   ) || (    \/|  \  ( |   ) (      ) (   | (   ) |
|  (_/ / | |   | || |      |   \ | |   | |      | |   | (___) |
|   _ (  | |   | || | ____ | (\ \) |   | |      | |   |  ___  |
|  ( \ \ | |   | || | \_  )| | \   |   | |      | |   | (   ) |
|  /  \ \| (___) || (___) || )  \  |___) (___   | |   | )   ( |
|_/    \/(_______)(_______)|/    )_)\_______/   )_(   |/     \|

@authors: free thinkers of the world
    1. Qua Is X (Ukraine) qua.is.kyiv.ua@gmail.com
    /add your name here.../

 */


use tonic::{transport::Server, Request, Response, Status};
use message::message_server::{Message, MessageServer};
use message::{MessageRequest, MessageResponse};
use log::{info};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use std::sync::Arc;
use std::sync::Mutex;
use libp2p::swarm::Swarm;
use super::super::node::p2p::PeerNetBehaviour;
pub mod message {
    tonic::include_proto!("message");
}

#[derive(Debug)]
pub struct MessageService {
    x: Arc<Mutex<UnboundedSender<String>>>
}

#[tonic::async_trait]
impl Message for MessageService {
    async fn post(&self, request: Request<MessageRequest>)
        ->Result<Response<MessageResponse>, Status>
    {
        println!("gRPC Request {:#?}", request);
        let _r = self.x.lock().unwrap().send(request.get_ref().message.clone()).unwrap();
        Ok(Response::new(MessageResponse{
            status_message: format!("request msg: {}", request.get_ref().message),
            status_code: 0,
        }))
    }
}

pub async fn run(port: Option<u16>, snd: Arc<Mutex<UnboundedSender<String>>>) -> Result<(), tonic::transport::Error> {
    info!("* gRPC service is running...");
    let _grpc_port = port.unwrap_or(50551);
    let addr = format!("0.0.0.0:{}", _grpc_port).parse().unwrap_or_else(
        |_| format!("::{}", _grpc_port).parse().unwrap()
    );
    
    let post_service = MessageService {
        x : snd
    };
    
    Server::builder()
        .add_service(MessageServer::new(post_service))
        .serve(addr).await
}