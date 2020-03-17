use crate::error::FluentError;
use serde::Serialize;
use std::time::SystemTime;
use tokio::{
    io::AsyncWriteExt,
    net::{
        TcpStream,
        ToSocketAddrs,
    },
};

pub struct FluentClient<A: ToSocketAddrs> {
    addr: A,
}

impl<A: ToSocketAddrs> FluentClient<A> {
    pub fn new(addr: A) -> Self {
        FluentClient {
            addr: addr,
        }
    }

    pub async fn message<S: Serialize>(&self, tag: &str, record: &S) -> Result<(), FluentError>  {
        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let record = serde_json::to_string(record)?;
        let body = format!("[{},{},{}]", tag, time, record);

        let mut stream = TcpStream::connect(&self.addr).await?;
        stream.write_all(&body.into_bytes()).await?;
        Ok(())
    }
}
