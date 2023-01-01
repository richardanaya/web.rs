var $7611355a66e759da$exports = {};
"use strict";
Object.defineProperty($7611355a66e759da$exports, "__esModule", {
    value: true
});
$7611355a66e759da$exports.ExternRef = void 0;
var $3aa6042011fa5456$exports = {};
"use strict";
Object.defineProperty($3aa6042011fa5456$exports, "__esModule", {
    value: true
});
$3aa6042011fa5456$exports.ExternRef = void 0;
var $0f8dde742b5c8611$exports = {};
"use strict";
Object.defineProperty($0f8dde742b5c8611$exports, "__esModule", {
    value: true
});
$0f8dde742b5c8611$exports.GenerationalArena = void 0;
const $0f8dde742b5c8611$var$MAX_GENERATION = 0xfffffff0;
class $0f8dde742b5c8611$var$GenerationalArena {
    constructor(){
        this.objects = [];
        this.generations = [];
        this.freeList = [];
        this.nextIndex = 0;
    }
    allocate(o) {
        let index;
        if (this.freeList.length > 0) index = this.freeList.pop();
        else index = this.nextIndex++;
        const currentGeneration = this.generations[index];
        this.objects[index] = o;
        this.generations[index] = currentGeneration === undefined ? 1 : Math.abs(currentGeneration) + 1;
        // return handle as big integer that contains
        // index in low 32 bits and generation in high 32 bits
        const low = BigInt(index);
        const high = BigInt(this.generations[index]) << BigInt(32);
        const merged = low | high;
        return merged;
    }
    deallocate(handle) {
        const index = Number(handle & BigInt(0xffffffff));
        const generation = Number(handle >> BigInt(32));
        if (generation >= $0f8dde742b5c8611$var$MAX_GENERATION) this.generations[index] = -this.generations[index];
        else if (generation === this.generations[index]) {
            this.generations[index] = -this.generations[index];
            this.freeList.push(index);
        } else throw new Error("attempt to deallocate invalid handle");
    }
    retrieve(handle) {
        const index = Number(handle & BigInt(0xffffffff));
        const generation = Number(handle >> BigInt(32));
        if (generation === this.generations[index]) return this.objects[index];
        else throw new Error("attempt to retrieve invalid handle");
    }
}
$0f8dde742b5c8611$exports.GenerationalArena = $0f8dde742b5c8611$var$GenerationalArena;


const $3aa6042011fa5456$var$store = new $0f8dde742b5c8611$exports.GenerationalArena();
class $3aa6042011fa5456$var$ExternRef {
    static create(reference) {
        return $3aa6042011fa5456$var$store.allocate(reference);
    }
    static load(handle) {
        return $3aa6042011fa5456$var$store.retrieve(handle);
    }
    static delete(handle) {
        $3aa6042011fa5456$var$store.deallocate(handle);
    }
}
$3aa6042011fa5456$exports.ExternRef = $3aa6042011fa5456$var$ExternRef;


Object.defineProperty($7611355a66e759da$exports, "ExternRef", {
    enumerable: true,
    get: function() {
        return $3aa6042011fa5456$exports.ExternRef;
    }
});


const $569963205592bc01$var$JsWasm = {
    createEnvironment () {
        (0, $7611355a66e759da$exports.ExternRef).create(undefined);
        (0, $7611355a66e759da$exports.ExternRef).create(null);
        (0, $7611355a66e759da$exports.ExternRef).create(self);
        (0, $7611355a66e759da$exports.ExternRef).create(typeof document != "undefined" ? document : null);
        (0, $7611355a66e759da$exports.ExternRef).create(typeof document != "undefined" ? document.body : null);
        // 0 is reserved for undefined
        // 1 is reserved for null
        // 2 is reserved for self
        // 3 is reserved for document
        // 4 is reserved for document.body
        const context = {
            functions: [
                function() {
                    debugger;
                    return 0;
                }
            ],
            utf8dec: new TextDecoder("utf-8"),
            utf8enc: new TextEncoder(),
            utf16dec: new TextDecoder("utf-16"),
            readUtf8FromMemory: function(start, len) {
                const text = this.utf8dec.decode(this.getMemory().subarray(start, start + len));
                return text;
            },
            createAllocation: function(size) {
                if (!this.module) throw new Error("module not set");
                const allocationId = this.module.instance.exports.create_allocation(size);
                const allocationPtr = this.module.instance.exports.allocation_ptr(allocationId);
                return [
                    allocationId,
                    allocationPtr
                ];
            },
            writeUtf8ToMemory: function(str) {
                const bytes = this.utf8enc.encode(str);
                const len = bytes.length;
                const [id, start] = this.createAllocation(len);
                this.getMemory().set(bytes, start);
                return id;
            },
            writeArrayBufferToMemory: function(ab) {
                const bytes = new Uint8Array(ab);
                const len = bytes.length;
                const [id, start] = this.createAllocation(len);
                this.getMemory().set(bytes, start);
                return id;
            },
            readUtf16FromMemory: function(start, len) {
                const text = this.utf16dec.decode(this.getMemory().subarray(start, start + len));
                return text;
            },
            readUint8ArrayFromMemory (start, length) {
                if (!this.module) throw new Error("module not set");
                const b = this.getMemory().slice(start, start + length);
                return new Uint8Array(b);
            },
            storeObject: function(obj) {
                return (0, $7611355a66e759da$exports.ExternRef).create(obj);
            },
            getObject: function(handle) {
                return (0, $7611355a66e759da$exports.ExternRef).load(handle);
            },
            releaseObject: function(handle) {
                // dont release our fixed references
                if (handle <= 4n) return;
                (0, $7611355a66e759da$exports.ExternRef).delete(handle);
            },
            getMemory: function() {
                if (!this.module) throw new Error("module not set");
                return new Uint8Array(this.module.instance.exports.memory.buffer);
            },
            readParameters: function(start, length) {
                //get bytes of parameters out of wasm module
                const parameters = this.readUint8ArrayFromMemory(start, length);
                //convert bytes to array of values  
                //assuming each paramter is preceded by a 32 bit integer indicating its type
                //0 = undefined
                //1 = null
                //2 = float-64
                //3 = bigint
                //4 = string (followed by 32-bit start and size of string in memory)
                //5 = extern ref
                //6 = array of float-64 (followed by 32-bit start and size of string in memory)
                //7 = true
                //8 = false
                const values = [];
                let i = 0;
                while(i < parameters.length){
                    const type = parameters[i];
                    i++;
                    switch(type){
                        case 0:
                            values.push(undefined);
                            break;
                        case 1:
                            values.push(null);
                            break;
                        case 2:
                            values.push(new DataView(parameters.buffer).getFloat64(i, true));
                            i += 8;
                            break;
                        case 3:
                            values.push(new DataView(parameters.buffer).getBigInt64(i, true));
                            i += 8;
                            break;
                        case 4:
                            {
                                const start1 = new DataView(parameters.buffer).getInt32(i, true);
                                i += 4;
                                const len = new DataView(parameters.buffer).getInt32(i, true);
                                i += 4;
                                values.push(context.readUtf8FromMemory(start1, len));
                                break;
                            }
                        case 5:
                            {
                                const handle = new DataView(parameters.buffer).getBigInt64(i, true);
                                values.push(context.getObject(handle));
                                i += 8;
                                break;
                            }
                        case 6:
                            {
                                const start2 = new DataView(parameters.buffer).getInt32(i, true);
                                i += 4;
                                const len1 = new DataView(parameters.buffer).getInt32(i, true);
                                i += 4;
                                const memory = context.getMemory();
                                const slice = memory.buffer.slice(start2, start2 + len1 * 4);
                                const array = new Float32Array(slice);
                                values.push(array);
                                break;
                            }
                        case 7:
                            values.push(true);
                            break;
                        case 8:
                            values.push(false);
                            break;
                        case 9:
                            {
                                const start3 = new DataView(parameters.buffer).getInt32(i, true);
                                i += 4;
                                const len2 = new DataView(parameters.buffer).getInt32(i, true);
                                i += 4;
                                const memory1 = context.getMemory();
                                const slice1 = memory1.buffer.slice(start3, start3 + len2 * 8);
                                const array1 = new Float64Array(slice1);
                                values.push(array1);
                                break;
                            }
                        case 10:
                            {
                                const start4 = new DataView(parameters.buffer).getInt32(i, true);
                                i += 4;
                                const len3 = new DataView(parameters.buffer).getInt32(i, true);
                                i += 4;
                                const memory2 = context.getMemory();
                                const slice2 = memory2.buffer.slice(start4, start4 + len3 * 4);
                                const array2 = new Uint32Array(slice2);
                                values.push(array2);
                                break;
                            }
                        default:
                            throw new Error("unknown parameter type");
                    }
                }
                return values;
            }
        };
        return [
            {
                abort () {
                    throw new Error("WebAssembly module aborted");
                },
                externref_drop (obj) {
                    context.releaseObject(obj);
                },
                js_register_function (start, len, utfByteLen) {
                    let functionBody;
                    if (utfByteLen === 16) functionBody = context.readUtf16FromMemory(start, len);
                    else functionBody = context.readUtf8FromMemory(start, len);
                    const id = context.functions.length;
                    context.functions.push(Function(`"use strict";return(${functionBody})`)());
                    return id;
                },
                js_invoke_function (funcHandle, parametersStart, parametersLength) {
                    const values = context.readParameters(parametersStart, parametersLength);
                    return context.functions[funcHandle].call(context, ...values);
                },
                js_invoke_function_and_return_object (funcHandle, parametersStart, parametersLength) {
                    const values = context.readParameters(parametersStart, parametersLength);
                    const result = context.functions[funcHandle].call(context, ...values);
                    if (result === undefined || result === null) throw new Error("js_invoke_function_and_return_object returned undefined or null while trying to return an object");
                    return context.storeObject(result);
                },
                js_invoke_function_and_return_bool (funcHandle, parametersStart, parametersLength) {
                    const values = context.readParameters(parametersStart, parametersLength);
                    const result = context.functions[funcHandle].call(context, ...values);
                    return result ? 1 : 0;
                },
                js_invoke_function_and_return_bigint (funcHandle, parametersStart, parametersLength) {
                    const values = context.readParameters(parametersStart, parametersLength);
                    const result = context.functions[funcHandle].call(context, ...values);
                    return result;
                },
                js_invoke_function_and_return_string (funcHandle, parametersStart, parametersLength) {
                    const values = context.readParameters(parametersStart, parametersLength);
                    const result = context.functions[funcHandle].call(context, ...values);
                    if (result === undefined || result === null) throw new Error("js_invoke_function_and_return_string returned undefined or null while trying to retrieve string.");
                    return context.writeUtf8ToMemory(result);
                },
                js_invoke_function_and_return_array_buffer (funcHandle, parametersStart, parametersLength) {
                    const values = context.readParameters(parametersStart, parametersLength);
                    const result = context.functions[funcHandle].call(context, ...values);
                    if (result === undefined || result === null) throw new Error("js_invoke_function_and_return_array_buffer returned undefined or null while trying to retrieve arraybuffer.");
                    return context.writeArrayBufferToMemory(result);
                }
            },
            context
        ];
    },
    async loadAndRunWasm (wasmURL) {
        const context = await this.load(wasmURL);
        context.module.instance.exports.main();
    },
    async load (wasmURL) {
        const [env, context] = $569963205592bc01$var$JsWasm.createEnvironment();
        const response = await fetch(wasmURL);
        const bytes = await response.arrayBuffer();
        const module = await WebAssembly.instantiate(bytes, {
            env: env
        });
        context.module = module;
        return context;
    }
};
document.addEventListener("DOMContentLoaded", function() {
    const wasmScripts = document.querySelectorAll("script[type='application/wasm']");
    for(let i = 0; i < wasmScripts.length; i++){
        const src = wasmScripts[i].src;
        if (src) $569963205592bc01$var$JsWasm.loadAndRunWasm(src);
        else console.error("Script tag must have 'src' property.");
    }
});


//# sourceMappingURL=js-wasm.js.map
