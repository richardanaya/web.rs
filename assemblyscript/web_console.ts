import * as jswasm from "./js-wasm"
let console_clear_fn:f64 = 0;
export function console_clear() : void {
    if( console_clear_fn === 0) {
        const code = `function(){ console.clear(); }`;
        console_clear_fn = <f64>jswasm.js_register_function(<f64>changetype<usize>(code),<f64>code.length*2, 16);
    }
    jswasm.js_invoke_function_0(console_clear_fn);
}

let console_log_fn:f64 = 0;
export function console_log(msg: string) : void {
    const a0: f64 = <f64>changetype<usize>(msg);
    const a1: f64 = msg.length*2;
    if( console_log_fn === 0) {
        const code = `function(msgPtr,msgLen){ console.log(this.readUtf16FromMemory(msgPtr,msgLen)); }`;
        console_log_fn = <f64>jswasm.js_register_function(<f64>changetype<usize>(code),<f64>code.length*2, 16);
    }
    jswasm.js_invoke_function_2(console_log_fn, a0, a1);
}

let console_warning_fn:f64 = 0;
export function console_warning(msg: string) : void {
    const a0: f64 = <f64>changetype<usize>(msg);
    const a1: f64 = msg.length*2;
    if( console_warning_fn === 0) {
        const code = `function(msgPtr,msgLen){ console.warn(this.readUtf16FromMemory(msgPtr,msgLen)); }`;
        console_warning_fn = <f64>jswasm.js_register_function(<f64>changetype<usize>(code),<f64>code.length*2, 16);
    }
    jswasm.js_invoke_function_2(console_warning_fn, a0, a1);
}

let console_error_fn:f64 = 0;
export function console_error(msg: string) : void {
    const a0: f64 = <f64>changetype<usize>(msg);
    const a1: f64 = msg.length*2;
    if( console_error_fn === 0) {
        const code = `function(msgPtr,msgLen){ console.error(this.readUtf16FromMemory(msgPtr,msgLen)); }`;
        console_error_fn = <f64>jswasm.js_register_function(<f64>changetype<usize>(code),<f64>code.length*2, 16);
    }
    jswasm.js_invoke_function_2(console_error_fn, a0, a1);
}

let console_time_fn:f64 = 0;
export function console_time(msg: string) : void {
    const a0: f64 = <f64>changetype<usize>(msg);
    const a1: f64 = msg.length*2;
    if( console_time_fn === 0) {
        const code = `function(msgPtr,msgLen){ console.time(this.readUtf16FromMemory(msgPtr,msgLen)); }`;
        console_time_fn = <f64>jswasm.js_register_function(<f64>changetype<usize>(code),<f64>code.length*2, 16);
    }
    jswasm.js_invoke_function_2(console_time_fn, a0, a1);
}

let console_time_end_fn:f64 = 0;
export function console_time_end(msg: string) : void {
    const a0: f64 = <f64>changetype<usize>(msg);
    const a1: f64 = msg.length*2;
    if( console_time_end_fn === 0) {
        const code = `function(msgPtr,msgLen){ console.timeEnd(this.readUtf16FromMemory(msgPtr,msgLen)); }`;
        console_time_end_fn = <f64>jswasm.js_register_function(<f64>changetype<usize>(code),<f64>code.length*2, 16);
    }
    jswasm.js_invoke_function_2(console_time_end_fn, a0, a1);
}

