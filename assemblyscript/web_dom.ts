import * as jswasm from "./js-wasm"
let document_get_element_by_id_fn:f64 = 0;
export function document_get_element_by_id(id: string) : f64 {
    const a0: f64 = <f64>changetype<usize>(id);
    const a1: f64 = id.length*2;
    if( document_get_element_by_id_fn === 0) {
        const code = `function(idPtr,idLen){ return  this.storeObject(document.getElementById(this.readUtf16FromMemory(idPtr,idLen))); }`;
        document_get_element_by_id_fn = <f64>jswasm.js_register_function(<f64>changetype<usize>(code),<f64>code.length*2, 16);
    }
    return jswasm.js_invoke_function_2(document_get_element_by_id_fn, a0, a1);
}

