use web::*;

#[no_mangle]
pub fn main() {
    let e = get_element_by_id("foo");
    let s = attach_shadow(&e,true);
    let t = get_attribute(&e,"bar");
    set_inner_html(&s,&t.unwrap());
}
