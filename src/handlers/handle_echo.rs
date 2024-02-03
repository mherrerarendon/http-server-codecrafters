use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{http_request::HttpRequest, http_response::HttpResponse, http_serde::HttpSerialize};

pub async fn handle_echo(mut stream: TcpStream, request: &HttpRequest) -> anyhow::Result<()> {
    let (_, response_text) = request.path[1..]
        .split_once("/")
        .ok_or(anyhow::anyhow!("Expected to find delimiter"))?;
    let mut response = HttpResponse::new_with_status(200);
    response.headers.add("Content-Type", "text/plain");
    response
        .headers
        .add("Content-Length", &response_text.len().to_string());
    response.body = response_text.to_string();

    let response_str = response.http_serialize();
    println!("response_str: {}", response_str);

    stream.write_all(response_str.as_bytes()).await?;
    Ok(())
}