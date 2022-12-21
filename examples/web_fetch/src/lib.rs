use web::*;
use std::sync::Arc;

#[no_mangle]
pub fn main() {
    let body = query_selector("body");
    
    let request = Arc::new(XMLHttpRequest::new());
    let r2 = request.clone();
    request.open("GET", "https://pokeapi.co/api/v2/pokemon/1/");
    request.send();
    request.set_on_load(move|| {
        let text = r2.response_text();
        element_set_inner_html(&body, &text);
    });
}
