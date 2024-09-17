use square::square_client::SquareClient;

pub mod square {
    tonic::include_proto!("square");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SquareClient::connect("http://0.0.0.0:50051").await?;

    let request = square::SquareRequest {
        name: "Some square".to_string(),
        width: 3,
        height: 4,
    };

    let response = client.calc_square(request).await?;
    println!("RESPONSE={:#?}", response);

    Ok(())
}
