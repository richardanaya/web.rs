use web::*;

#[no_mangle]
pub fn main() {
    set_inner_html(BODY,"Hello World!");
}
