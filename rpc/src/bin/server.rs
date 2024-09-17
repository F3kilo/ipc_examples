use square::square_server::{Square, SquareServer};
use square::{SquareRequest, SquareResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod square {
    tonic::include_proto!("square");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let service = MySquareService::default();

    Server::builder()
        .add_service(SquareServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
struct MySquareService {}

#[tonic::async_trait]
impl Square for MySquareService {
    async fn calc_square(
        &self,
        request: Request<SquareRequest>,
    ) -> Result<Response<SquareResponse>, Status> {
        println!("Got a request: {:#?}", request);

        let request = request.into_inner();
        let square = request.width * request.height;
        let message = format!("The square of {} is {}", request.name, square);

        Ok(Response::new(SquareResponse { square, message }))
    }
}
