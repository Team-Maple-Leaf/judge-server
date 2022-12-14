use tracing::info;
use tracing_subscriber;

mod consumer;
mod executor;
mod filter;
mod judge;
mod publisher;

fn main() {
    #[cfg(debug_assertions)]
    info!("test");
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    let addr = "amqp://rabbitmq:5672/%2f";

    let consume_channel = consumer::create_channel(addr);

    consumer::consume(consume_channel);
}
