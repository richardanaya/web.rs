#![no_std]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
pub use externref_polyfill::ExternRef;
use raw_parts::RawParts;
use spin::Mutex;

pub const JS_UNDEFINED: ExternRef = ExternRef { value: 0 };
pub const JS_NULL: ExternRef = ExternRef { value: 1 };
pub const DOM_SELF: ExternRef = ExternRef { value: 2 };
pub const DOM_WINDOW: ExternRef = ExternRef { value: 2 };
pub const DOM_DOCUMENT: ExternRef = ExternRef { value: 3 };
pub const DOM_BODY: ExternRef = ExternRef { value: 4 };

extern "C" {
    fn js_register_function(start: f64, len: f64) -> f64;
    fn js_invoke_function(
        fn_handle: f64,
        parameters_start: *const u8,
        parameters_length: usize,
    ) -> f64;
    fn js_invoke_function_and_return_object(
        fn_handle: f64,
        parameters_start: *const u8,
        parameters_length: usize,
    ) -> i64;
    fn js_invoke_function_and_return_bigint(
        fn_handle: f64,
        parameters_start: *const u8,
        parameters_length: usize,
    ) -> i64;
    fn js_invoke_function_and_return_string(
        fn_handle: f64,
        parameters_start: *const u8,
        parameters_length: usize,
    ) -> usize;
    fn js_invoke_function_and_return_array_buffer(
        fn_handle: f64,
        parameters_start: *const u8,
        parameters_length: usize,
    ) -> usize;
    fn js_invoke_function_and_return_bool(
        fn_handle: f64,
        parameters_start: *const u8,
        parameters_length: usize,
    ) -> f64;
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
pub enum InvokeParam<'a> {
    Undefined,
    Null,
    Float64(f64),
    BigInt(i64),
    String(&'a str),
    ExternRef(&'a ExternRef),
    Float32Array(&'a [f32]),
    Float64Array(&'a [f64]),
    Bool(bool),
    Uint32Array(&'a [u32]),
}

impl From<f64> for InvokeParam<'_> {
    fn from(f: f64) -> Self {
        InvokeParam::Float64(f)
    }
}

impl From<i32> for InvokeParam<'_> {
    fn from(i: i32) -> Self {
        InvokeParam::Float64(i as f64)
    }
}

impl From<usize> for InvokeParam<'_> {
    fn from(i: usize) -> Self {
        InvokeParam::Float64(i as f64)
    }
}

impl From<i64> for InvokeParam<'_> {
    fn from(i: i64) -> Self {
        InvokeParam::BigInt(i)
    }
}

impl<'a> From<&'a str> for InvokeParam<'a> {
    fn from(s: &'a str) -> Self {
        InvokeParam::String(s)
    }
}

impl<'a> From<&'a ExternRef> for InvokeParam<'a> {
    fn from(i: &'a ExternRef) -> Self {
        InvokeParam::ExternRef(i)
    }
}

impl<'a> From<&'a [f32]> for InvokeParam<'a> {
    fn from(a: &'a [f32]) -> Self {
        InvokeParam::Float32Array(a)
    }
}

impl<'a> From<&'a [f64]> for InvokeParam<'a> {
    fn from(a: &'a [f64]) -> Self {
        InvokeParam::Float64Array(a)
    }
}

impl From<bool> for InvokeParam<'_> {
    fn from(b: bool) -> Self {
        InvokeParam::Bool(b)
    }
}

impl<'a> From<&'a [u32]> for InvokeParam<'a> {
    fn from(a: &'a [u32]) -> Self {
        InvokeParam::Uint32Array(a)
    }
}

fn param_to_bytes(params: &[InvokeParam]) -> Vec<u8> {
    let mut param_bytes = Vec::new();
    for param in params {
        match param {
            InvokeParam::Undefined => {
                param_bytes.push(0);
            }
            InvokeParam::Null => {
                param_bytes.push(1);
            }
            InvokeParam::Float64(f) => {
                param_bytes.push(2);
                param_bytes.extend_from_slice(&f.to_le_bytes());
            }
            InvokeParam::BigInt(i) => {
                param_bytes.push(3);
                param_bytes.extend_from_slice(&i.to_le_bytes());
            }
            InvokeParam::String(s) => {
                param_bytes.push(4);
                let start = s.as_ptr() as usize;
                let len = s.len();
                param_bytes.extend_from_slice(&start.to_le_bytes());
                param_bytes.extend_from_slice(&len.to_le_bytes());
            }
            InvokeParam::ExternRef(i) => {
                param_bytes.push(5);
                param_bytes.extend_from_slice(&i.value.to_le_bytes());
            }
            InvokeParam::Float32Array(a) => {
                param_bytes.push(6);
                let start = a.as_ptr() as usize;
                let len = a.len();
                param_bytes.extend_from_slice(&start.to_le_bytes());
                param_bytes.extend_from_slice(&len.to_le_bytes());
            }
            InvokeParam::Bool(b) => {
                if *b {
                    param_bytes.push(7);
                } else {
                    param_bytes.push(8);
                }
            }
            InvokeParam::Float64Array(a) => {
                param_bytes.push(9);
                let start = a.as_ptr() as usize;
                let len = a.len();
                param_bytes.extend_from_slice(&start.to_le_bytes());
                param_bytes.extend_from_slice(&len.to_le_bytes());
            }
            InvokeParam::Uint32Array(a) => {
                param_bytes.push(10);
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
    pub fn invoke(&self, params: &[InvokeParam]) -> f64
where {
        let param_bytes = param_to_bytes(params);
        let RawParts {
            ptr,
            length,
            capacity: _,
        } = RawParts::from_vec(param_bytes);
        unsafe { js_invoke_function(self.fn_handle, ptr, length) }
    }

    pub fn invoke_and_return_object(&self, params: &[InvokeParam]) -> ExternRef
where {
        let param_bytes = param_to_bytes(params);
        let RawParts {
            ptr,
            length,
            capacity: _,
        } = RawParts::from_vec(param_bytes);
        let handle = unsafe { js_invoke_function_and_return_object(self.fn_handle, ptr, length) };
        ExternRef { value: handle }
    }

    pub fn invoke_and_return_bigint(&self, params: &[InvokeParam]) -> i64
where {
        let param_bytes = param_to_bytes(params);
        let RawParts {
            ptr,
            length,
            capacity: _,
        } = RawParts::from_vec(param_bytes);
        unsafe { js_invoke_function_and_return_bigint(self.fn_handle, ptr, length) }
    }

    pub fn invoke_and_return_string(&self, params: &[InvokeParam]) -> String
where {
        let param_bytes = param_to_bytes(params);
        let RawParts {
            ptr,
            length,
            capacity: _,
        } = RawParts::from_vec(param_bytes);
        let allocation_id =
            unsafe { js_invoke_function_and_return_string(self.fn_handle, ptr, length) };
        extract_string_from_memory(allocation_id)
    }

    pub fn invoke_and_return_array_buffer(&self, params: &[InvokeParam]) -> Vec<u8>
where {
        let param_bytes = param_to_bytes(params);
        let RawParts {
            ptr,
            length,
            capacity: _,
        } = RawParts::from_vec(param_bytes);
        let allocation_id =
            unsafe { js_invoke_function_and_return_array_buffer(self.fn_handle, ptr, length) };
        extract_vec_from_memory(allocation_id)
    }

    pub fn invoke_and_return_bool(&self, params: &[InvokeParam]) -> bool {
        let param_bytes = param_to_bytes(params);
        let RawParts {
            ptr,
            length,
            capacity: _,
        } = RawParts::from_vec(param_bytes);
        let ret = unsafe { js_invoke_function_and_return_bool(self.fn_handle, ptr, length) };
        ret != 0.0
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

static ALLOCATIONS: Mutex<Vec<Option<Vec<u8>>>> = Mutex::new(Vec::new());

pub fn extract_string_from_memory(allocation_id: usize) -> String {
    let allocations = ALLOCATIONS.lock();
    let allocation = allocations.get(allocation_id).unwrap();
    let vec = allocation.as_ref().unwrap();
    let s = String::from_utf8(vec.clone()).unwrap();
    s
}

pub fn extract_vec_from_memory(allocation_id: usize) -> Vec<u8> {
    let allocations = ALLOCATIONS.lock();
    let allocation = allocations.get(allocation_id).unwrap();
    let vec = allocation.as_ref().unwrap();
    vec.clone()
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
