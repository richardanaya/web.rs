use js::*;

fn query_selector(selector: &str) -> ExternRef {
    let query_selector = js!(r#"
        function(selector){
            return document.querySelector(selector);
        }"#);
    query_selector.invoke_and_return_object(&[selector.into()])
}

fn add_click_listener(element: &ExternRef, callback: &str) {
    let add_click_listener = js!(r#"
        function(element, callback){
            element.addEventListener("click", ()=>{
                this.module.instance.exports[callback]();
            });
        }"#);
    add_click_listener.invoke(&[element.into(), callback.into()]);
}

fn element_set_inner_html(element: &ExternRef, html: &str) {
    let set_inner_html = js!(r#"
        function(element, html){
            element.innerHTML = html;
        }"#);
    set_inner_html.invoke(&[element.into(), html.into()]);
}

fn fetch(url: &str, callback: &str) {
    let fetch = js!(r#"
        function(url, callback){
            fetch(url).then((response)=>{
                return response.text();
            }).then((text)=>{
                const allocationId = this.writeUtf8ToMemory(text);
                this.module.instance.exports[callback](allocationId);
            });
        }"#);
    fetch.invoke(&[url.into(), callback.into()]);
}

#[no_mangle]
pub fn main() {
    let button = query_selector("#fetch_button");
    add_click_listener(&button, "button_clicked");
}

#[no_mangle]
pub fn button_clicked() {
    // get pokemon data
    let url = "https://pokeapi.co/api/v2/pokemon/1/";
    fetch(url, "fetch_callback");
}

#[no_mangle]
pub fn fetch_callback(text_allocation_id: usize) {
    let text = extract_string_from_memory(text_allocation_id);
    let result = query_selector("#data_output");
    element_set_inner_html(&result, &text);
}
