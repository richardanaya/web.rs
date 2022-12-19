use js::*;

fn query_selector(selector: &str) -> ExternRef {
    let query_selector = js!(r#"
        function(selector){
            return document.querySelector(selector);
        }"#);
    query_selector.invoke_and_return_object(&[selector.into()])
}

fn canvas_get_context(canvas: &ExternRef) -> ExternRef {
    let get_context = js!(r#"
        function(canvas){
            return canvas.getContext("2d");
        }"#);
    get_context.invoke_and_return_object(&[canvas.into()])
}

fn canvas_set_fill_style(ctx: &ExternRef, color: &str) {
    let set_fill_style = js!(r#"
        function(ctx, color){
            ctx.fillStyle = color;
        }"#);
    set_fill_style.invoke(&[ctx.into(), color.into()]);
}

fn canvas_fill_rect(ctx: &ExternRef, x: f64, y: f64, width: f64, height: f64) {
    let fill_rect = js!(r#"
        function(ctx, x, y, width, height){
            ctx.fillRect(x, y, width, height);
        }"#);
    fill_rect.invoke(&[ctx.into(), x.into(), y.into(), width.into(), height.into()]);
}

#[no_mangle]
pub fn main() {
    let screen = query_selector("#screen");
    let ctx = &canvas_get_context(&screen);
    canvas_set_fill_style(&ctx, "red");
    canvas_fill_rect(&ctx, 10.0, 10.0, 100.0, 100.0);
    canvas_set_fill_style(&ctx, "green");
    canvas_fill_rect(&ctx, 20.0, 20.0, 100.0, 100.0);
    canvas_set_fill_style(&ctx, "blue");
    canvas_fill_rect(&ctx, 30.0, 30.0, 100.0, 100.0);
}
