use tonic::{transport::Server, Request, Response, Status};

use short_link::short_link_server::{ShortLink, ShortLinkServer};

use short_link::{ShortLinkReply, ShortLinkRequest};

pub mod short_link {
    tonic::include_proto!("shortlink");
}

#[derive(Debug, Default)]
pub struct MyShortLink {}

#[tonic::async_trait]
impl ShortLink for MyShortLink {
    async fn get_info(
        &self,
        request: Request<ShortLinkRequest>,
    ) -> Result<Response<ShortLinkReply>, Status> {
        println!("Got a request: {:?}", request);

        // todo: 需要实现根据request.id来查询数据库和读redis, 这些都是上次公开课说过的, 所以就不写了
        let reply = short_link::ShortLinkReply {
            url: String::from("http://www.baidu.com"),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse()?;
    let shortlink = MyShortLink::default();

    Server::builder()
        .add_service(ShortLinkServer::new(shortlink))
        .serve(addr)
        .await?;

    Ok(())
}