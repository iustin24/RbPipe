# RbPipe

RbPipe is a simple command-line tool that reads data from STDIN and sends it to a RabbitMQ queue.

## Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)
- RabbitMQ server

## Installation

1. Clone the repository:

```
git clone https://github.com/iustin24/RbPipe
cd rbpipe
```

2. Build the binary:

```
cargo build --release
```

3. Add the compiled binary to your system's PATH or copy it to a location in your PATH.

## Usage

Before running RbPipe, make sure to set the `RABBITMQ_HOST` environment variable to the address of your RabbitMQ server:

```bash
export RABBITMQ_HOST="your.rabbitmq.host"
```

Replace "your.rabbitmq.host" with the hostname or IP address of your RabbitMQ server.

To use RbPipe, pipe the output of a command to the RbPipe binary, providing the queue name as an argument:

```
cat message.txt | rbpipe my_queue
```

