

@external("env", "js_register_function")
export declare function js_register_function(codePtr: f64, codeLen: f64, utfByeLen: f64): f64
@external("env", "js_invoke_function")
export declare function js_invoke_function(fn: f64, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64, i: f64, j: f64): f64
@external("env", "js_release")
export declare function js_release(ref: f64): void
