# Rust Key-Value Store

A simple key-value store written in rust using the tokio runtime.
## Table of Contents
- [How to use this app](#how-to-use-this-app)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
- [How it works](#how-it-works)
    - [Commands](#commands)
    - [Examples](#examples)
- [License](#license)   
- [Acknowledgments](#acknowledgments)




## How to use this app

### Prerequisites
<!--  redis-cli -->
- [redis-cli](https://redis.io/topics/rediscli) installed on your machine
<!--  rust -->
- [rust](https://www.rust-lang.org/tools/install) installed on your machine

### Installation
<!--  clone this repo -->
- Clone this repo to your local machine using `
git clone
`
<!--  cd into the repo -->
- `cd` into the repo
<!--  install dependencies -->
- Install dependencies using `cargo install`

### Usage
<!--  run the app -->
- Run the app using `cargo run`
<!--  build the app -->
- Build the app using `cargo build`

## How it works
<!--  explain how the app works tokio runtuime -->
This app uses the [tokio](https://tokio.rs/) runtime to handle asynchronous tasks. The tokio runtime is a low-level, zero-cost abstraction over [futures](https://docs.rs/futures/0.3.5/futures/). It provides several tools to help you write asynchronous code, including:
- The `tokio::spawn` function for executing a future on a thread pool
- Asynchronous versions of common std types, including TCP streams (`tokio::net::TcpStream`), UDP sockets (`tokio::net::UdpSocket`), and channels (`tokio::sync::mpsc` and `tokio::sync::oneshot`)
- The `tokio::sync::Mutex` type, an asynchronous version of `std::sync::Mutex`
- Timer types, including `tokio::time::Instant` and `tokio::time::Duration`
- The `tokio::main` macro, which makes it easy to run a tokio application

<!--  explain how to use the app -->
This app is a simple key-value store that allows you to set, get, and delete keys. It uses the [redis protocol](https://redis.io/topics/protocol) to communicate with the redis-cli. The redis-cli is a command line interface for redis that allows you to interact with the redis server. The redis-cli is a simple program that allows you to send commands to the server and read the replies sent back.  

The redis protocol is a text-based protocol that uses the client-server model. The client sends a command to the server and the server responds with a reply. The client and server communicate using a TCP connection. The client sends a command to the server and the server responds with a reply. T

### Commands

<!-- make the above into a table -->
| Command | Description |
| --- | --- |
| `redis-cli set key value` | Set a key |
| `redis-cli get key` | Get a key |
| `redis-cli set key value ex time` | Set an expiration time for a key |
| `redis-cli ttl key` | Get the remaining time for a key |
| `redis-cli del key` | Delete a key |
| `redis-cli flush` | Delete all keys |

### Examples

<!-- make the above into a table -->
| Command | Description |
| --- | --- |
| `redis-cli set name john` | Set the key `name` to the value `john` |
| `redis-cli get name` | Get the value of the key `name` |
| `redis-cli set name john ex 10` | Set the key `name` to the value `john` and set an expiration time of 10 seconds |
| `redis-cli ttl name` | Get the remaining time for the key `name` |
| `redis-cli del name` | Delete the key `name` |
| `redis-cli flush` | Delete all keys |

## License
<!--  MIT -->
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## Acknowledgments
<!--  tokio -->
- [tokio](https://tokio.rs/)
<!--  redis -->
- [redis](https://redis.io/)
<!--  rust -->
- [rust](https://www.rust-lang.org/)

<!--  buil your own org -->
- [Build your own org](
    https://build-your-own.org/redis
)
<!--  redis protocol -->
- [Redis protocol](https://redis.io/topics/protocol)








