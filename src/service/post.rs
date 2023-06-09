
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

pub mod message {
    tonic::include_proto!("message");
}

#[derive(Debug, Default)]
pub struct MessageService {
    
}

#[tonic::async_trait]
impl Message for MessageService {
    async fn post(&self, request: Request<MessageRequest>)
        ->Result<Response<MessageResponse>, Status>
    {
        println!("gRPC Request {:#?}", request);
        Ok(Response::new(MessageResponse{
            status_message: format!("request msg: {}", request.get_ref().message),
            status_code: 0,
        }))
    }
}

pub async fn run(port: Option<u16>)  {
    let _grpc_port = port.unwrap_or(50551);
    let addr = format!("0.0.0.0:{}", _grpc_port).parse().unwrap_or_else(
        |_| format!("::{}", _grpc_port).parse().unwrap()
    );
    let post_service = MessageService::default();

    let future = Server::builder()
        .add_service(MessageServer::new(post_service))
        .serve(addr);
    future.await.unwrap_or_else(
        |e| {
            eprintln!("server error: {}", e);
            std::process::exit(1);
        },
    );
}