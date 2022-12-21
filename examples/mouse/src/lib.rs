use web::*;

#[no_mangle]
pub fn main() {
    let body = query_selector("body");
    element_add_click_listener(&body, |x, y| {
        console_log(format!("Clicked at {}, {}", x, y).as_str());
    });
}
