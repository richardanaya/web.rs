"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const externref_polyfill_1 = require("externref_polyfill");
const JsWasm = {
    createEnvironment() {
        externref_polyfill_1.ExternRef.create(undefined);
        externref_polyfill_1.ExternRef.create(null);
        externref_polyfill_1.ExternRef.create(self);
        externref_polyfill_1.ExternRef.create(typeof document != "undefined" ? document : null);
        externref_polyfill_1.ExternRef.create(typeof document != "undefined" ? document.body : null);
        // 0 is reserved for undefined
        // 1 is reserved for null
        // 2 is reserved for self
        // 3 is reserved for document
        // 4 is reserved for document.body
        const context = {
            functions: [
                function () {
                    debugger;
                    return 0;
                },
            ],
            utf8dec: new TextDecoder("utf-8"),
            utf8enc: new TextEncoder(),
            utf16dec: new TextDecoder("utf-16"),
            readUtf8FromMemory: function (start, len) {
                const text = this.utf8dec.decode(this.getMemory().subarray(start, start + len));
                return text;
            },
            createAllocation: function (size) {
                if (!this.module) {
                    throw new Error("module not set");
                }
                const allocationId = this.module.instance.exports.create_allocation(size);
                const allocationPtr = this.module.instance.exports.allocation_ptr(allocationId);
                return [allocationId, allocationPtr];
            },
            writeUtf8ToMemory: function (str) {
                const bytes = this.utf8enc.encode(str);
                const len = bytes.length;
                const [id, start] = this.createAllocation(len);
                this.getMemory().set(bytes, start);
                return id;
            },
            readUtf16FromMemory: function (start, len) {
                const text = this.utf16dec.decode(this.getMemory().subarray(start, start + len));
                return text;
            },
            readUint8ArrayFromMemory(start) {
                if (!this.module) {
                    throw new Error("module not set");
                }
                const data32 = new Uint32Array(this.module.instance.exports.memory.buffer);
                const ptr = data32[start / 4];
                const length = data32[ptr / 4];
                const b = this.getMemory().slice(ptr + 4, ptr + 4 + length);
                return new Uint8Array(b);
            },
            storeObject: function (obj) {
                return externref_polyfill_1.ExternRef.create(obj);
            },
            getObject: function (handle) {
                return externref_polyfill_1.ExternRef.load(handle);
            },
            releaseObject: function (handle) {
                externref_polyfill_1.ExternRef.delete(handle);
            },
            getMemory: function () {
                if (!this.module) {
                    throw new Error("module not set");
                }
                return new Uint8Array(this.module.instance.exports.memory.buffer);
            },
            readParameters: function (start) {
                //get bytes of parameters out of wasm module
                const parameters = this.readUint8ArrayFromMemory(start);
                //convert bytes to array of values  
                //assuming each paramter is preceded by a 32 bit integer indicating its type
                //0 = undefined
                //1 = null
                //2 = float-64
                //3 = bigint
                //4 = string (followed by 32-bit start and size of string in memory)
                //5 = extern ref
                //6 = array of float-64 (followed by 32-bit start and size of string in memory)
                const values = [];
                let i = 0;
                while (i < parameters.length) {
                    const type = parameters[i];
                    i++;
                    switch (type) {
                        case 0:
                            values.push(undefined);
                            i += 4;
                            break;
                        case 1:
                            values.push(null);
                            i += 4;
                            break;
                        case 2:
                            values.push(new DataView(parameters.buffer).getFloat64(i, true));
                            i += 4;
                            break;
                        case 3:
                            values.push(new BigInt64Array(parameters.buffer, i, 1)[0]);
                            i += 8;
                            break;
                        case 4: {
                            const start = new DataView(parameters.buffer).getInt32(i, true);
                            i += 4;
                            const len = new DataView(parameters.buffer).getInt32(i, true);
                            i += 4;
                            values.push(context.readUtf8FromMemory(start, len));
                            break;
                        }
                        case 5: {
                            const handle = new BigInt64Array(parameters.buffer, i, 1)[0];
                            values.push(context.getObject(handle));
                            i += 8;
                            break;
                        }
                        case 6: {
                            const start = new DataView(parameters.buffer).getInt32(i, true);
                            i += 4;
                            const len = new DataView(parameters.buffer).getInt32(i, true);
                            i += 4;
                            values.push(new Float64Array(parameters.buffer, start, len));
                            break;
                        }
                        default:
                            throw new Error("unknown parameter type");
                    }
                }
                return values;
            }
        };
        return [{
                abort() {
                    throw new Error("WebAssembly module aborted");
                },
                js_release(obj) {
                    context.releaseObject(obj);
                },
                js_register_function(start, len, utfByteLen) {
                    let functionBody;
                    if (utfByteLen === 16) {
                        functionBody = context.readUtf16FromMemory(start, len);
                    }
                    else {
                        functionBody = context.readUtf8FromMemory(start, len);
                    }
                    const id = context.functions.length;
                    context.functions.push(Function(`"use strict";return(${functionBody})`)());
                    return id;
                },
                js_invoke_function(funcHandle, parametersStart) {
                    const values = context.readParameters(parametersStart);
                    return context.functions[funcHandle].call(context, ...values);
                },
                js_invoke_function_and_return_object(funcHandle, parametersStart) {
                    const values = context.readParameters(parametersStart);
                    const result = context.functions[funcHandle].call(context, ...values);
                    return context.storeObject(result);
                },
            }, context];
    },
    async loadAndRunWasm(wasmURL) {
        const context = await this.load(wasmURL);
        context.module.instance.exports.main();
    },
    async load(wasmURL) {
        const [env, context] = JsWasm.createEnvironment();
        const response = await fetch(wasmURL);
        const bytes = await response.arrayBuffer();
        const module = await WebAssembly.instantiate(bytes, {
            env,
        });
        context.module = module;
        return context;
    },
};
document.addEventListener("DOMContentLoaded", function () {
    const wasmScripts = document.querySelectorAll("script[type='application/wasm']");
    for (let i = 0; i < wasmScripts.length; i++) {
        const src = wasmScripts[i].src;
        if (src) {
            JsWasm.loadAndRunWasm(src);
        }
        else {
            console.error("Script tag must have 'src' property.");
        }
    }
});
