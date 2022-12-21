use web::*;

#[no_mangle]
pub fn main() {
    let body = query_selector("body");
    element_add_click_listener(&body, |e| {
        console_log(format!("Clicked at {}, {}", e.offset_x, e.offset_y).as_str());
    });
}
