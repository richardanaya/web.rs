

@external("env", "js_register_function")
export declare function js_register_function(codePtr: f64, codeLen: f64, utfByeLen: f64): f64
@external("env", "js_invoke_function")
export declare function js_invoke_function(fn: f64, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64, i: f64, j: f64): f64
@external("env", "js_release")
export declare function js_release(ref: f64): void

export function js_invoke_0(fn: f64): void {
    js_invoke_function(fn,0,0,0,0,0,0,0,0,0,0);
}

export function js_invoke_1(fn: f64, a: f64): void {
    js_invoke_function(fn,a,0,0,0,0,0,0,0,0,0);
}

export function js_invoke_2(fn: f64, a: f64, b: f64): void {
    js_invoke_function(fn,a,b,0,0,0,0,0,0,0,0);
}

export function js_invoke_3(fn: f64, a: f64, b: f64, c: f64): void {
    js_invoke_function(fn,a,b,c,0,0,0,0,0,0,0);
}

export function js_invoke_4(fn: f64, a: f64, b: f64, c: f64, d: f64): void {
    js_invoke_function(fn,a,b,c,d,0,0,0,0,0,0);
}

export function js_invoke_5(fn: f64, a: f64, b: f64, c: f64, d: f64, e: f64): void {
    js_invoke_function(fn,a,b,c,d,e,0,0,0,0,0);
}

export function js_invoke_6(fn: f64, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64): void {
    js_invoke_function(fn,a,b,c,d,e,f,0,0,0,0);
}

export function js_invoke_7(fn: f64, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64): void {
    js_invoke_function(fn,a,b,c,d,e,f,g,0,0,0);
}

export function js_invoke_8(fn: f64, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64): void {
    js_invoke_function(fn,a,b,c,d,e,f,g,h,0,0);
}

export function js_invoke_9(fn: f64, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64, i: f64): void {
    js_invoke_function(fn,a,b,c,d,e,f,g,h,i,0);
}

export function js_invoke_10(fn: f64,  a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64, i: f64, j: f64): void {
    js_invoke_function(fn,a,b,c,d,e,f,g,h,i,j);
}