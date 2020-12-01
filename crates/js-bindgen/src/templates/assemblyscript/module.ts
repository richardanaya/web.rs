import {js_register_function,js_invoke_function} from "./js-wasm"

let console_log_fn:f64 = 0;
export function console_log(msg: string) : void {
  if(console_log_fn === 0){
    const code = "function(msgPtr,msgLen){ console.log(this.readUtf16FromMemory(msgPtr,msgLen)); }";
    console_log_fn = <f64>js_register_function(<f64>changetype<usize>(code),<f64>code.length*2, 16);
  }
  js_invoke_function(<f64>console_log_fn,<f64>changetype<usize>(msg),<f64>msg.length*2,0,0,0,0,0,0,0,0)
}
