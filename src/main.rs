use amiquip::{Connection, Exchange, Publish, Result};
use std::env;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <queue_name>", args[0]);
        std::process::exit(1);
    }

    let queue_name = &args[1];

    // Read RabbitMQ host from environment variable.
    let rabbitmq_host = match env::var("RABBITMQ_HOST") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Please set the RABBITMQ_HOST environment variable");
            std::process::exit(1);
        }
    };

    // Connect to RabbitMQ server.
    let mut connection = Connection::insecure_open(&format!("amqp://{}", rabbitmq_host))?;
    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare(queue_name, Default::default())?;
    let exchange = Exchange::direct(&channel);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            // Send message to the specified RabbitMQ queue
            exchange.publish(Publish::new(line.as_bytes(), queue.name()))?;
        }
    }

    // Close the connection when done.
    connection.close()
}
