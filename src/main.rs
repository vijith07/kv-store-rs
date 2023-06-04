use anyhow::Result;
use resp::Resp::{BulkString, Null, SimpleString};
use std::{
    error::Error,
    sync::{Arc, Mutex},
};
use store::Store;
use tokio::net::{TcpListener, TcpStream};
mod resp;
mod store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let main_store = Arc::new(Mutex::new(Store::new()));

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((socket, _)) => {
                print!("Accepted connection from {:?}", socket.peer_addr()?);
                let client_store = main_store.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_client(socket, client_store).await {
                        println!("an error occurred; error = {:?}", e);
                    }
                });
            }
            Err(e) => {
                println!("failed to accept socket; error = {:?}", e);
            }
        }
    }
}

async fn handle_client(stream: TcpStream, client_store: Arc<Mutex<Store>>) -> Result<()> {
    let mut conn = resp::RespCodec::new(stream);
    loop {
        let bytes_read = conn.read_resp().await?;
        if let Some(bytes_read) = bytes_read {
            let (command, args) = bytes_read.to_command()?;
            println!("command: {}, args: {:?}", command, args);
            let response = match command.to_ascii_lowercase().as_ref() {
                "ping" => resp::Resp::SimpleString("PONG".to_string()),
                "echo" => args.first().unwrap().clone(),
                "get" => {
                    if let Some(BulkString(key)) = args.first() {
                        if let Some(value) = client_store.lock().unwrap().get(key.clone()) {
                            SimpleString(value)
                        } else {
                            Null
                        }
                    } else {
                        resp::Resp::Error("Get requires a key".to_string())
                    }
                }
                "set" => {
                    if let (Some(BulkString(key)), Some(BulkString(value))) =
                        (args.get(0), args.get(1))
                    {
                        if let (Some(BulkString(_)), Some(BulkString(amount))) =
                            (args.get(2), args.get(3))
                        {
                            client_store.lock().unwrap().set_with_expiry(
                                key.clone(),
                                value.clone(),
                                amount.parse::<u64>()?,
                            );
                        } else {
                            client_store.lock().unwrap().set(key.clone(), value.clone());
                        }
                        resp::Resp::SimpleString("OK".to_string())
                    } else {
                        resp::Resp::Error("Set requires a key and a value".to_string())
                    }
                }
                "del" => {
                    if let Some(BulkString(key)) = args.first() {
                        client_store.lock().unwrap().del(key.clone());
                        resp::Resp::SimpleString("OK".to_string())
                    } else {
                        resp::Resp::Error("Del requires a key".to_string())
                    }
                }
                "exists" => {
                    if let Some(BulkString(key)) = args.first() {
                        if client_store.lock().unwrap().exists(key.clone()) {
                            resp::Resp::Integer(1)
                        } else {
                            resp::Resp::Integer(0)
                        }
                    } else {
                        resp::Resp::Error("Exists requires a key".to_string())
                    }
                }
                "expire" => {
                    if let (Some(BulkString(key)), Some(BulkString(amount))) =
                        (args.get(0), args.get(1))
                    {
                        if client_store
                            .lock()
                            .unwrap()
                            .expire(key.clone(), amount.parse::<u64>()?)
                        {
                            resp::Resp::Integer(1)
                        } else {
                            resp::Resp::Integer(0)
                        }
                    } else {
                        resp::Resp::Error("Expire requires a key and an amount".to_string())
                    }
                }
                "ttl" => {
                    if let Some(BulkString(key)) = args.first() {
                        if let Some(ttl) = client_store.lock().unwrap().ttl(key.clone()) {
                            resp::Resp::Integer(ttl as i64)
                        } else {
                            resp::Resp::Integer(-1)
                        }
                    } else {
                        resp::Resp::Error("TTL requires a key".to_string())
                    }
                }

                "persist" => {
                    if let Some(BulkString(key)) = args.first() {
                        if client_store.lock().unwrap().persist(key.clone()) {
                            resp::Resp::Integer(1)
                        } else {
                            resp::Resp::Integer(0)
                        }
                    } else {
                        resp::Resp::Error("Persist requires a key".to_string())
                    }
                }

                "keys" => {
                    let keys = client_store.lock().unwrap().keys();
                    resp::Resp::Array(
                        keys.into_iter()
                            .map(|key| BulkString(key))
                            .collect::<Vec<resp::Resp>>(),
                    )
                }

                "flushdb" => {
                    client_store.lock().unwrap().flush();
                    resp::Resp::SimpleString("OK".to_string())
                }
                _ => resp::Resp::Error("unknown command".to_string()),
            };
            conn.write_resp(response).await?;
        } else {
            break;
        }
    }
    Ok(())
}
