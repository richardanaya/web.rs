use js::*;

#[no_mangle]
pub fn main() {
    let query_selector = js!(r#"
        function(selector){
            return document.querySelector(selector);
        }"#);

    let screen = &query_selector.invoke_and_return_object(&["#screen".into()]);

    let get_context = js!(r#"
        function(el){
            debugger;
            return el.getContext("2d");
        }"#);

    let ctx = &get_context.invoke_and_return_object(&[screen.into()]);

    let set_fill_style = js!(r#"
        function(ctx, color){
            debugger;
            ctx.fillStyle = color;
        }"#);

    let fill_rect = js!(r#"
        function(ctx, x, y, w, h){
            debugger;
            ctx.fillRect(x, y, w, h);
        }"#);

    set_fill_style.invoke(&[ctx.into(), "red".into()]);
    fill_rect.invoke(&[ctx.into(), 10.into(), 10.into(), 100.into(), 100.into()]);

    set_fill_style.invoke(&[ctx.into(), "green".into()]);
    fill_rect.invoke(&[ctx.into(), 20.into(), 20.into(), 100.into(), 100.into()]);

    set_fill_style.invoke(&[ctx.into(), "blue".into()]);
    fill_rect.invoke(&[ctx.into(), 30.into(), 30.into(), 100.into(), 100.into()]);
}
