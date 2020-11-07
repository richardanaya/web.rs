#[no_mangle]
pub fn main() {
    web_timer::set_interval(||{
        web_console::log("⏰");
    }, 1000);
}
