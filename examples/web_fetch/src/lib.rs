use web::*;

#[no_mangle]
pub fn main() {
    let body = query_selector("body");

    fetch(
        "https://pokeapi.co/api/v2/pokemon/1/",
        HTTPMethod::GET,
        None,
        None,
        move |_status, text| {
            element_set_inner_html(&body, &text);
        },
    );
}
