pub use html_color::*;
pub use js::*;
mod common;
pub use common::*;
mod util;
pub use util::*;
mod console;
pub use console::*;
mod canvas;
pub use canvas::*;
mod local_storage;
pub use local_storage::*;
mod http_request;
pub use http_request::*;
mod history;
pub use history::*;
mod request_animation_frame;
pub use request_animation_frame::*;
mod dom;
pub use dom::*;
pub mod executor;

pub fn run_event_loop() {
    executor::poll_tasks();
    request_animation_frame(run_event_loop);
}
pub use web_macro::main;
