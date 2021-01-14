use anyhow::Result;
use futures::io::WriteHalf;
use futures::prelude::*;
use smol::Async;
use std::net::TcpStream;

async fn send(mut writer: WriteHalf<Async<TcpStream>>) -> Result<()> {
    writer.write_all(&[0xb0, 0x0b, 0x1e, 0x55]).await?;
    Ok(())
}

async fn connect() -> Result<()> {
    match Async::<TcpStream>::connect(([127, 0, 0, 1], 7000)).await {
        Ok(stream) => {
            let (mut reader, writer) = stream.split();
            let send_task = smol::spawn(send(writer));

            println!("connected!");
            let mut magic = [0u8; 4];
            reader.read_exact(&mut magic).await?;
            println!("read {:x?}", magic);

            send_task.await?;
        }
        Err(err) => {
            eprintln!("couldn't connect {}", err);
        }
    };
    Ok(())
}

fn main() -> Result<()> {
    println!("client");
    smol::block_on(connect())
}

