use web::*;

#[no_mangle]
pub fn main() {
    set_interval(
        || {
            log(&format!("⏰ {}", random()));
        },
        1000,
    );
}
