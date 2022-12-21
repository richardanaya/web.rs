use web::*;

#[no_mangle]
pub fn main() {
    let body = query_selector("body");
    element_add_click_listener(&body, |e| {
        console_log(format!("Clicked at {}, {}", e.offset_x, e.offset_y).as_str());
    });
    element_add_mouse_move_listener(&body, |e| {
        console_log(format!("Mouse moved to {}, {}", e.offset_x, e.offset_y).as_str());
    });
    element_add_mouse_down_listener(&body, |e| {
        console_log(format!("Mouse down at {}, {}", e.offset_x, e.offset_y).as_str());
    });
    element_add_mouse_up_listener(&body, |e| {
        console_log(format!("Mouse up at {}, {}", e.offset_x, e.offset_y).as_str());
    });
    element_add_key_down_listener(&body, |e| {
        console_log(format!("Key down: {}", e.key_code).as_str());
    });
    element_add_key_up_listener(&body, |e| {
        console_log(format!("Key up: {}", e.key_code).as_str());
    });
}
