#![no_std]
extern crate alloc;
use alloc::vec::Vec;
pub use cstring::cstr_to_string;

pub const JS_NULL: f64 = 0.0;
pub const JS_UNDEFINED: f64 = 1.0;
pub const DOM_SELF: f64 = 2.0;
pub const DOM_WINDOW: f64 = 2.0;
pub const DOM_DOCUMENT: f64 = 3.0;
pub const DOM_BODY: f64 = 4.0;

extern "C" {
    fn js_register_function(start: f64, len: f64) -> f64;
    fn js_release(obj: f64);
    fn js_invoke_function(
        fn_handle: f64,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
        g: f64,
        h: f64,
        i: f64,
        j: f64,
    ) -> f64;
}

#[derive(Copy, Clone)]
pub struct JSFunction {
    fn_handle: f64,
}

impl From<f64> for JSFunction {
    fn from(f: f64) -> Self {
        JSFunction { fn_handle: f }
    }
}

impl Into<f64> for JSFunction {
    fn into(self) -> f64 {
        self.fn_handle
    }
}

impl Into<f64> for &JSFunction {
    fn into(self) -> f64 {
        self.fn_handle
    }
}

impl JSFunction {
    pub fn invoke_0(&self) -> f64
where {
        unsafe {
            js_invoke_function(
                self.fn_handle,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            )
        }
    }

    pub fn invoke_1<A>(&self, a: A) -> f64
    where
        A: Into<f64>,
    {
        unsafe {
            js_invoke_function(
                self.fn_handle,
                a.into(),
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            )
        }
    }

    pub fn invoke_2<A, B>(&self, a: A, b: B) -> f64
    where
        A: Into<f64>,
        B: Into<f64>,
    {
        unsafe {
            js_invoke_function(
                self.fn_handle,
                a.into(),
                b.into(),
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            )
        }
    }

    pub fn invoke_3<A, B, C>(&self, a: A, b: B, c: C) -> f64
    where
        A: Into<f64>,
        B: Into<f64>,
        C: Into<f64>,
    {
        unsafe {
            js_invoke_function(
                self.fn_handle,
                a.into(),
                b.into(),
                c.into(),
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            )
        }
    }

    pub fn invoke_4<A, B, C, D>(&self, a: A, b: B, c: C, d: D) -> f64
    where
        A: Into<f64>,
        B: Into<f64>,
        C: Into<f64>,
        D: Into<f64>,
    {
        unsafe {
            js_invoke_function(
                self.fn_handle,
                a.into(),
                b.into(),
                c.into(),
                d.into(),
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            )
        }
    }

    pub fn invoke_5<A, B, C, D, E>(&self, a: A, b: B, c: C, d: D, e: E) -> f64
    where
        A: Into<f64>,
        B: Into<f64>,
        C: Into<f64>,
        D: Into<f64>,
        E: Into<f64>,
    {
        unsafe {
            js_invoke_function(
                self.fn_handle,
                a.into(),
                b.into(),
                c.into(),
                d.into(),
                e.into(),
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            )
        }
    }

    pub fn invoke_6<A, B, C, D, E, F>(&self, a: A, b: B, c: C, d: D, e: E, f: F) -> f64
    where
        A: Into<f64>,
        B: Into<f64>,
        C: Into<f64>,
        D: Into<f64>,
        E: Into<f64>,
        F: Into<f64>,
    {
        unsafe {
            js_invoke_function(
                self.fn_handle,
                a.into(),
                b.into(),
                c.into(),
                d.into(),
                e.into(),
                f.into(),
                0.0,
                0.0,
                0.0,
                0.0,
            )
        }
    }

    pub fn invoke_7<A, B, C, D, E, F, G>(&self, a: A, b: B, c: C, d: D, e: E, f: F, g: G) -> f64
    where
        A: Into<f64>,
        B: Into<f64>,
        C: Into<f64>,
        D: Into<f64>,
        E: Into<f64>,
        F: Into<f64>,
        G: Into<f64>,
    {
        unsafe {
            js_invoke_function(
                self.fn_handle,
                a.into(),
                b.into(),
                c.into(),
                d.into(),
                e.into(),
                f.into(),
                g.into(),
                0.0,
                0.0,
                0.0,
            )
        }
    }

    pub fn invoke_8<A, B, C, D, E, F, G, H>(
        &self,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
        f: F,
        g: G,
        h: H,
    ) -> f64
    where
        A: Into<f64>,
        B: Into<f64>,
        C: Into<f64>,
        D: Into<f64>,
        E: Into<f64>,
        F: Into<f64>,
        G: Into<f64>,
        H: Into<f64>,
    {
        unsafe {
            js_invoke_function(
                self.fn_handle,
                a.into(),
                b.into(),
                c.into(),
                d.into(),
                e.into(),
                f.into(),
                g.into(),
                h.into(),
                0.0,
                0.0,
            )
        }
    }

    pub fn invoke_9<A, B, C, D, E, F, G, H, I>(
        &self,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
        f: F,
        g: G,
        h: H,
        i: I,
    ) -> f64
    where
        A: Into<f64>,
        B: Into<f64>,
        C: Into<f64>,
        D: Into<f64>,
        E: Into<f64>,
        F: Into<f64>,
        G: Into<f64>,
        H: Into<f64>,
        I: Into<f64>,
    {
        unsafe {
            js_invoke_function(
                self.fn_handle,
                a.into(),
                b.into(),
                c.into(),
                d.into(),
                e.into(),
                f.into(),
                g.into(),
                h.into(),
                i.into(),
                0.0,
            )
        }
    }

    pub fn invoke_10<A, B, C, D, E, F, G, H, I, J>(
        &self,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
        f: F,
        g: G,
        h: H,
        i: I,
        j: J,
    ) -> f64
    where
        A: Into<f64>,
        B: Into<f64>,
        C: Into<f64>,
        D: Into<f64>,
        E: Into<f64>,
        F: Into<f64>,
        G: Into<f64>,
        H: Into<f64>,
        I: Into<f64>,
        J: Into<f64>,
    {
        unsafe {
            js_invoke_function(
                self.fn_handle,
                a.into(),
                b.into(),
                c.into(),
                d.into(),
                e.into(),
                f.into(),
                g.into(),
                h.into(),
                i.into(),
                j.into(),
            )
        }
    }
}

pub fn register_function(code: &str) -> JSFunction {
    let start = code.as_ptr();
    let len = code.len();
    unsafe {
        JSFunction {
            fn_handle: js_register_function(start as usize as f64, len as f64),
        }
    }
}

pub struct JSObject {
    pub handle: f64,
}

impl Into<f64> for JSObject {
    fn into(self) -> f64 {
        self.handle
    }
}

impl Into<f64> for &JSObject {
    fn into(self) -> f64 {
        self.handle
    }
}

impl From<f64> for JSObject {
    fn from(n: f64) -> Self {
        JSObject { handle: n }
    }
}

impl Drop for JSObject {
    fn drop(&mut self) {
        unsafe {
            js_release(self.handle);
        }
    }
}

#[no_mangle]
fn malloc(size: i32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

#[macro_export]
macro_rules! js {
    ($e:expr) => {{
        lazy_static::lazy_static! {
        static ref FN: js::JSFunction= {
            js::register_function(
                "function(handler,time){
                        window.setTimeout(this.createCallback(handler),time);
                    }",
            )
        };};
        &FN
    }};
}
