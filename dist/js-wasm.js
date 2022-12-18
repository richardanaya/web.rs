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
                let b = this.getMemory().slice(ptr + 4, ptr + 4 + length);
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
                js_invoke_function(funcHandle, a, b, c, d, e, f, g, h, i, j) {
                    return context.functions[funcHandle].call(context, a, b, c, d, e, f, g, h, i, j);
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
