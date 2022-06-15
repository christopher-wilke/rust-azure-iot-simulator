use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let mut request = tonic::Request::new(HelloRequest {
        name: "Christopher".into(),
    });

    let response = client.say_hello(request).await?;

    println!("Response received: {:?}", response);

    Ok(())
}