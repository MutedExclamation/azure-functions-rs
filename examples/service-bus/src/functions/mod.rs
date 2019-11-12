// WARNING: This file is regenerated by the `cargo func new` command.

mod create_queue_message;
mod create_topic_message;
mod log_queue_message;
mod log_topic_message;

// Export the Azure Functions here.
azure_functions::export! {
    create_queue_message::create_queue_message,
    create_topic_message::create_topic_message,
    log_queue_message::log_queue_message,
    log_topic_message::log_topic_message,
}
