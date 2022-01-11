use grpc::ClientStubExt;
use rpc::square::SquareRequest;
use rpc::square_grpc::SquareClient;

fn main() {
    let client = SquareClient::new_plain("127.0.0.1", 55331, Default::default()).unwrap();

    let request = SquareRequest {
        name: "small rectangle".into(),
        width: 5,
        height: 3,
        ..Default::default()
    };

    let response_fut = client
        .calc_square(Default::default(), request)
        .join_metadata_result();
    let response = pollster::block_on(response_fut);
    println!("{:#?}", response)
}
