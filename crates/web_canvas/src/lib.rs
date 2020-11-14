#![no_std]
use js::*;

pub struct CanvasContext {
    pub handle: f64,
}

pub trait Canvas2dApi {
    fn set_fill_color(&self, color: &str);
    fn fill_rect(&self, x: impl Into<f64>, y: impl Into<f64>, w: impl Into<f64>, h: impl Into<f64>);
}

impl CanvasContext {
    pub fn from_canvas_element(el: impl Into<f64>) -> CanvasContext {
        CanvasContext {
            handle: {
                static FN: once_cell::sync::OnceCell<JSFunction> = once_cell::sync::OnceCell::new();
                FN.get_or_init(|| {
                    register_function(
                        r#"function(el){   
                            el = this.getObject(el);
                            let ctx = el.getContext("2d");
                            return this.storeObject(ctx);
                        }"#,
                    )
                })
                .invoke_1(el.into())
            },
        }
    }
}

impl Canvas2dApi for CanvasContext {
    fn set_fill_color(&self, color: &str) {
        lazy_static::lazy_static! {
            static ref FN: JSFunction= {
            register_function(
                "function(ctx,strPtr,strLen){
                                ctx = this.getObject(ctx);
                                ctx.fillStyle = this.readUtf8FromMemory(strPtr,strLen);
                        }",
            )
        };};
        FN.invoke_3(self.handle, color.as_ptr() as u32, color.len() as u32);
    }

    fn fill_rect(
        &self,
        x: impl Into<f64>,
        y: impl Into<f64>,
        w: impl Into<f64>,
        h: impl Into<f64>,
    ) {
        lazy_static::lazy_static! {
            static ref FN: JSFunction= {
            register_function(
                "function(ctx,x,y,w,h){
                                ctx = this.getObject(ctx);
                                ctx.fillRect(x,y,w,h);
                        }",
            )
        };};
        FN.invoke_5(self.handle, x, y, w, h);
    }
}
