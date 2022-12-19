use web::*;

#[no_mangle]
pub fn main() {
    set_inner_html(DOM_BODY,"Hello World!");
}
