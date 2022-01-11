use grpc::{ServerBuilder, ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};
use rpc::square::{SquareRequest, SquareResponse};
use rpc::square_grpc::{Square, SquareServer};
use std::thread;

fn main() {
    let mut server = ServerBuilder::new_plain();
    server.http.set_addr("0.0.0.0:55331").unwrap();
    server.add_service(SquareServer::new_service_def(SquareImpl));
    let _runnung = server.build().unwrap();

    loop {
        thread::park()
    }
}

struct SquareImpl;

impl Square for SquareImpl {
    fn calc_square(
        &self,
        _: ServerHandlerContext,
        req: ServerRequestSingle<SquareRequest>,
        resp: ServerResponseUnarySink<SquareResponse>,
    ) -> grpc::Result<()> {
        let name = req.message.name;
        let width = req.message.width;
        let height = req.message.height;

        let square = width * height;
        let message = format!("Calculated square for {}: {}", name, square);
        println!("{}", message);
        let response = SquareResponse {
            message,
            square,
            ..Default::default()
        };

        resp.finish(response)
    }
}
