#![no_std]
use js::*;

pub struct CanvasContext {
    pub handle: f64,
}

pub trait Canvas2dApi {
    fn set_fill_color(&self, color: &str);
    fn fill_rect(&self, x: impl Into<f64>, y: impl Into<f64>, w: impl Into<f64>, h: impl Into<f64>);
    fn clear_rect(
        &self,
        x: impl Into<f64>,
        y: impl Into<f64>,
        w: impl Into<f64>,
        h: impl Into<f64>,
    );
}

impl CanvasContext {
    pub fn from_canvas_element(el: impl Into<f64>) -> CanvasContext {
        CanvasContext {
            handle: {
                js!(r#"function(el){
                    el = this.getObject(el);
                    let ctx = el.getContext("2d");
                    return this.storeObject(ctx);
                }"#)
                .invoke_1(el.into())
            },
        }
    }
}

impl Canvas2dApi for CanvasContext {
    fn set_fill_color(&self, color: &str) {
        js!("function(ctx,strPtr,strLen){
            ctx = this.getObject(ctx);
            ctx.fillStyle = this.readUtf8FromMemory(strPtr,strLen);
        }")
        .invoke_3(self.handle, color.as_ptr() as u32, color.len() as u32);
    }

    fn fill_rect(
        &self,
        x: impl Into<f64>,
        y: impl Into<f64>,
        w: impl Into<f64>,
        h: impl Into<f64>,
    ) {
        js!("function(ctx,x,y,w,h){
            ctx = this.getObject(ctx);
            ctx.fillRect(x,y,w,h);
        }")
        .invoke_5(self.handle, x, y, w, h);
    }

    fn clear_rect(
        &self,
        x: impl Into<f64>,
        y: impl Into<f64>,
        w: impl Into<f64>,
        h: impl Into<f64>,
    ) {
        js!("function(ctx,x,y,w,h){
            ctx = this.getObject(ctx);
            ctx.clearRect(x,y,w,h);
        }")
        .invoke_5(self.handle, x, y, w, h);
    }
}
