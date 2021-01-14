use anyhow::Result;
use futures::io::WriteHalf;
use futures::prelude::*;
use smol::Async;
use std::net::{TcpStream, TcpListener};

async fn serve(mut writer: WriteHalf<Async<TcpStream>>) -> Result<()> {
    writer.write_all(&[0xde, 0xad, 0xbe, 0xef]).await?;
    Ok(())
}

async fn listen() -> Result<()> {
    let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 7000))?;

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        println!("Accepted client: {}", peer_addr);

        let (mut reader, writer) = stream.split();

        let send_task = smol::spawn(serve(writer));

        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic).await?;
        println!("read {:x?}", magic);

        send_task.await?;
    }
}

fn main() -> Result<()> {
    println!("server");
    smol::block_on(listen())
}

