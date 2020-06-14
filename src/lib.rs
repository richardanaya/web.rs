

pub const NULL:usize = 0;
pub const UNDEFINED:usize = 1;
pub const SELF:usize = 2;
pub const DOCUMENT:usize = 3;

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

pub struct JSInvoker {
    fn_handle: usize,
}

impl JSInvoker {
    pub fn invoke_0<R>(&self) -> R
    where
        R: From<f64>,
    {
        unsafe {
            R::from(js_invoke_function(
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
            ))
        }
    }

    pub fn invoke_1<A, R>(&self, a: A) -> R
    where
        R: From<f64>,
        A: Into<f64>,
    {
        unsafe {
            R::from(js_invoke_function(
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
            ))
        }
    }

    pub fn invoke_2<A, B, R>(&self, a: A, b: B) -> R
    where
        R: From<f64>,
        A: Into<f64>,
        B: Into<f64>,
    {
        unsafe {
            R::from(js_invoke_function(
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
            ))
        }
    }

    pub fn invoke_3<A, B, C, R>(&self, a: A, b: B, c: C) -> R
    where
        R: From<f64>,
        A: Into<f64>,
        B: Into<f64>,
        C: Into<f64>,
    {
        unsafe {
            R::from(js_invoke_function(
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
            ))
        }
    }

    pub fn invoke_4<A, B, C, D, R>(&self, a: A, b: B, c: C, d: D) -> R
    where
        R: From<f64>,
        A: Into<f64>,
        B: Into<f64>,
        C: Into<f64>,
        D: Into<f64>,
    {
        unsafe {
            R::from(js_invoke_function(
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
            ))
        }
    }

    pub fn invoke_5<A, B, C, D, E, R>(&self, a: A, b: B, c: C, d: D, e: E) -> R
    where
        R: From<f64>,
        A: Into<f64>,
        B: Into<f64>,
        C: Into<f64>,
        D: Into<f64>,
        E: Into<f64>,
    {
        unsafe {
            R::from(js_invoke_function(
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
            ))
        }
    }

    pub fn invoke_6<A, B, C, D, E, F, R>(&self, a: A, b: B, c: C, d: D, e: E, f: F) -> R
    where
        R: From<f64>,
        A: Into<f64>,
        B: Into<f64>,
        C: Into<f64>,
        D: Into<f64>,
        E: Into<f64>,
        F: Into<f64>,
    {
        unsafe {
            R::from(js_invoke_function(
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
            ))
        }
    }

    pub fn invoke_7<A, B, C, D, E, F, G, R>(&self, a: A, b: B, c: C, d: D, e: E, f: F, g: G) -> R
    where
        R: From<f64>,
        A: Into<f64>,
        B: Into<f64>,
        C: Into<f64>,
        D: Into<f64>,
        E: Into<f64>,
        F: Into<f64>,
        G: Into<f64>,
    {
        unsafe {
            R::from(js_invoke_function(
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
            ))
        }
    }

    pub fn invoke_8<A, B, C, D, E, F, G, H, R>(
        &self,
        a: A,
        b: B,
        c: C,
        d: D,
        e: E,
        f: F,
        g: G,
        h: H,
    ) -> R
    where
        R: From<f64>,
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
            R::from(js_invoke_function(
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
            ))
        }
    }

    pub fn invoke_9<A, B, C, D, E, F, G, H, I, R>(
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
    ) -> R
    where
        R: From<f64>,
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
            R::from(js_invoke_function(
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
            ))
        }
    }

    pub fn invoke_10<A, B, C, D, E, F, G, H, I, J, R>(
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
    ) -> R
    where
        R: From<f64>,
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
            R::from(js_invoke_function(
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
            ))
        }
    }
}

pub fn register_function(code: &str) -> JSInvoker {
    let start = code.as_ptr();
    let len = code.len();
    unsafe {
        JSInvoker {
            fn_handle: js_register_function(start as usize, len),
        }
    }
}
