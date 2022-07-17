#![no_std]
#![allow(clippy::too_many_arguments)]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
pub use callback::*;
use spin::Mutex;

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
    pub fn_handle: f64,
}

impl From<f64> for JSFunction {
    fn from(f: f64) -> Self {
        JSFunction { fn_handle: f }
    }
}

impl From<&JSFunction> for f64 {
    fn from(f: &JSFunction) -> Self {
        f.fn_handle
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

impl From<&JSObject> for f64 {
    fn from(f: &JSObject) -> Self {
        f.handle
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

static ALLOCATIONS: Mutex<Vec<Option<Vec<u8>>>> = Mutex::new(Vec::new());

pub fn extract_string_from_memory(allocation_id: usize) -> String {
    let allocations = ALLOCATIONS.lock();
    let allocation = allocations.get(allocation_id).unwrap();
    let vec = allocation.as_ref().unwrap();
    String::from_utf8(vec.clone()).unwrap()
}

#[no_mangle]
pub fn create_allocation(size: i32) -> usize {
    let buf = Vec::with_capacity(size as usize);
    let mut allocations = ALLOCATIONS.lock();
    let i = allocations.len();
    allocations.push(Some(buf));
    i
}

#[no_mangle]
pub fn allocation_ptr(allocation_id: i32) -> *const u8 {
    let allocations = ALLOCATIONS.lock();
    let allocation = allocations.get(allocation_id as usize).unwrap();
    let vec = allocation.as_ref().unwrap();
    vec.as_ptr()
}

#[no_mangle]
pub fn allocation_len(allocation_id: i32) -> f64 {
    let allocations = ALLOCATIONS.lock();
    let allocation = allocations.get(allocation_id as usize).unwrap();
    let vec = allocation.as_ref().unwrap();
    vec.len() as f64
}

pub fn clear_allocation(allocation_id: usize) {
    let mut allocations = ALLOCATIONS.lock();
    allocations[allocation_id] = None;
}

#[macro_export]
macro_rules! js {
    ($e:expr) => {{
        static mut FN: Option<f64> = None;
        unsafe {
            if FN.is_none() {
                FN = Some(js::register_function($e).fn_handle);
            }
            JSFunction {
                fn_handle: FN.unwrap(),
            }
        }
    }};
}
