#![no_std]
#![allow(clippy::too_many_arguments)]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;
use raw_parts::RawParts;

pub const JS_UNDEFINED: f64 = 0.0;
pub const JS_NULL: f64 = 1.0;
pub const DOM_SELF: f64 = 2.0;
pub const DOM_WINDOW: f64 = 2.0;
pub const DOM_DOCUMENT: f64 = 3.0;
pub const DOM_BODY: f64 = 4.0;

extern "C" {
    fn js_register_function(start: f64, len: f64) -> f64;
    fn js_release(obj: f64);
    fn js_invoke_function(fn_handle: f64, parameters_start: *const u8, parameters_length: usize) -> f64;
    fn js_invoke_function_and_return_object(fn_handle: f64, parameters_start: *const u8, parameters_length: usize) -> i64;
}

#[derive(Copy, Clone)]
pub struct JSFunction {
    pub fn_handle: f64,
}

//convert invoke parameters into bytes
//assuming each parameter is preceded by a 32 bit integer indicating its type
//0 = undefined
//1 = null
//2 = float-64
//3 = bigint
//4 = string (followed by 32-bit start and size of string in memory)
//5 = extern ref
//6 = array of float-64 (followed by 32-bit start and size of string in memory)
pub enum InvokeParams<'a> {
    Undefined,
    Null,
    Float64(f64),
    BigInt(i64),
    String(&'a str),
    ExternRef(i64),
    Array(&'a [f64]),
}

fn param_to_bytes(params: &Vec<InvokeParams>) -> Vec<u8> {
    let mut param_bytes = Vec::new();
    for param in params {
        match param {
            InvokeParams::Undefined => {
                param_bytes.push(0);
            }
            InvokeParams::Null => {
                param_bytes.push(1);
            }
            InvokeParams::Float64(f) => {
                param_bytes.push(2);
                param_bytes.extend_from_slice(&f.to_le_bytes());
            }
            InvokeParams::BigInt(i) => {
                param_bytes.push(3);
                param_bytes.extend_from_slice(&i.to_le_bytes());
            }
            InvokeParams::String(s) => {
                param_bytes.push(4);
                let start = s.as_ptr() as usize;
                let len = s.len();
                param_bytes.extend_from_slice(&start.to_le_bytes());
                param_bytes.extend_from_slice(&len.to_le_bytes());
            }
            InvokeParams::ExternRef(i) => {
                param_bytes.push(5);
                param_bytes.extend_from_slice(&i.to_le_bytes());
            }
            InvokeParams::Array(a) => {
                param_bytes.push(6);
                let start = a.as_ptr() as usize;
                let len = a.len();
                param_bytes.extend_from_slice(&start.to_le_bytes());
                param_bytes.extend_from_slice(&len.to_le_bytes());
            }
        }
    }
    param_bytes
}

impl JSFunction {
    pub fn invoke(&self, params: &Vec<InvokeParams>) -> f64
where {
        let param_bytes = param_to_bytes(params);
        let RawParts { ptr, length, capacity: _ } = RawParts::from_vec(param_bytes);
        unsafe { js_invoke_function(self.fn_handle, ptr, length) }
    }

    pub fn invoke_and_return_object(&self, params: &Vec<InvokeParams>) -> i64
where {
        let param_bytes = param_to_bytes(params);
        let RawParts { ptr, length, capacity: _ } = RawParts::from_vec(param_bytes);
        unsafe { js_invoke_function_and_return_object(self.fn_handle, ptr, length) }
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
pub fn create_allocation(size: usize) -> usize {
    let mut buf = Vec::with_capacity(size as usize);
    buf.resize(size, 0);
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
