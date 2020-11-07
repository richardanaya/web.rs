#![no_std]

pub const JS_NULL: u32 = 0;
pub const JS_UNDEFINED: u32 = 1;
pub const DOM_SELF: u32 = 2;
pub const DOM_WINDOW: u32 = 2;
pub const DOM_DOCUMENT: u32 = 3;
pub const DOM_BODY: u32 = 4;

extern "C" {
    fn js_register_function(start: usize, len: usize) -> usize;
    fn js_invoke_function(
        fn_handle: usize,
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

pub struct JSFunction {
    fn_handle: usize,
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
            fn_handle: js_register_function(start as usize, len),
        }
    }
}
