import { ExternRef } from '../node_modules/externref_polyfill/';

interface JSWasmHandlerContext {
  functions: ((...args: unknown[]) => number)[];
  utf8dec: TextDecoder;
  utf8enc: TextEncoder;
  utf16dec: TextDecoder;
  storeObject: (obj: unknown) => bigint;
  releaseObject: (objHandle: bigint) => void;
  module?: WebAssembly.WebAssemblyInstantiatedSource;
  readUtf8FromMemory: (start: number, end: number) => string;
  readUtf16FromMemory: (start: number, end: number) => string;
  getMemory: () => Uint8Array;
  createAllocation: (size: number) => [number,number] ;
  writeUtf8ToMemory: (txt: string) => number;
  writeArrayBufferToMemory: (ab: ArrayBuffer) => number;
  readUint8ArrayFromMemory: (start: number, length: number) => Uint8Array;
  getObject: (handle: bigint) => unknown;
  readParameters: (start: number, length: number) => unknown[];
}

const JsWasm = {
  createEnvironment(): [WebAssembly.ModuleImports, JSWasmHandlerContext] {
    ExternRef.create(undefined);
    ExternRef.create(null);
    ExternRef.create(self);
    ExternRef.create(typeof document != "undefined" ? document : null);
    ExternRef.create(typeof document != "undefined" ? document.body : null);

    // 0 is reserved for undefined
    // 1 is reserved for null
    // 2 is reserved for self
    // 3 is reserved for document
    // 4 is reserved for document.body


    const context: JSWasmHandlerContext = {
      functions: [
        function () {
          debugger;
          return 0;
        },
      ],
      utf8dec: new TextDecoder("utf-8"),
      utf8enc: new TextEncoder(),
      utf16dec: new TextDecoder("utf-16"),
      readUtf8FromMemory: function (start: number, len: number) {
        const text = this.utf8dec.decode(
          this.getMemory().subarray(start, start + len)
        );
        return text;
      },
      createAllocation: function (size: number): [number,number] {
        if (!this.module) {
          throw new Error("module not set");
        }
        const allocationId = (this.module.instance.exports.create_allocation as (
          size: number
        ) => number)(size);
        const allocationPtr = (this.module.instance.exports.allocation_ptr as (
          size: number
        ) => number)(allocationId);
        return [allocationId, allocationPtr];
      },
      writeUtf8ToMemory: function (str: string) {
        const bytes = this.utf8enc.encode(str);
        const len = bytes.length;
        const [id,start] = this.createAllocation(len);
        this.getMemory().set(bytes, start);
        return id;
      },
      writeArrayBufferToMemory: function (ab: ArrayBuffer) {
        const bytes = new Uint8Array(ab);
        const len = bytes.length;
        const [id,start] = this.createAllocation(len);
        this.getMemory().set(bytes, start);
        return id;
      },
      readUtf16FromMemory: function (start: number, len: number) {
        const text = this.utf16dec.decode(
          this.getMemory().subarray(start, start + len)
        );
        return text;
      },
      readUint8ArrayFromMemory(start: number, length: number) {
        if (!this.module) {
          throw new Error("module not set");
        }
        const b = this.getMemory().slice(start,start+length);
        return new Uint8Array(b);
      },
      storeObject: function (obj: unknown) : bigint {
        return ExternRef.create(obj);
      },
      getObject: function (handle: bigint) {
        return ExternRef.load(handle);
      },
      releaseObject: function (handle: bigint) {
        // dont release our fixed references
        if(handle <= 4n) {
          return;
        }
        ExternRef.delete(handle);
      },
      getMemory: function () {
        if (!this.module) {
          throw new Error("module not set");
        }
        return new Uint8Array(
          (this.module.instance.exports.memory as WebAssembly.Memory).buffer
        );
      },
      readParameters: function (start: number, length: number) {
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
        
        const values: unknown[] = [];
        let i = 0;
        while (i < parameters.length) {
          const type = parameters[i];
          i++;
          switch (type) {
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
              values.push(new DataView(parameters.buffer).getBigInt64(i,true));
              i += 8;
              break;
            case 4: {
              const start = new DataView(parameters.buffer).getInt32(i, true);
              i += 4;
              const len = new DataView(parameters.buffer).getInt32(i, true);
              i += 4;
              values.push(
                context.readUtf8FromMemory(start, len)
              );
              break;
            }
            case 5: {
              const handle = new DataView(parameters.buffer).getBigInt64(i,true);
              values.push(context.getObject(handle));
              i += 8;
              break;
            }
            case 6: {
              const start = new DataView(parameters.buffer).getInt32(i, true);
              i += 4;
              const len = new DataView(parameters.buffer).getInt32(i, true);
              i += 4;
              const memory = context.getMemory();
              const slice = memory.buffer.slice(start, start + len * 4);
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
            case 9: {
              const start = new DataView(parameters.buffer).getInt32(i, true);
              i += 4;
              const len = new DataView(parameters.buffer).getInt32(i, true);
              i += 4;
              const memory = context.getMemory();
              const slice = memory.buffer.slice(start, start + len * 8);
              const array = new Float64Array(slice);
              values.push(array);
              break;
            }
            case 10: {
              const start = new DataView(parameters.buffer).getInt32(i, true);
              i += 4;
              const len = new DataView(parameters.buffer).getInt32(i, true);
              i += 4;
              const memory = context.getMemory();
              const slice = memory.buffer.slice(start, start + len * 4);
              const array = new Uint32Array(slice);
              values.push(array);
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
      externref_drop(obj: bigint) {
        context.releaseObject(obj);
      },
      js_register_function(start: number, len: number, utfByteLen: number) {
        let functionBody;
        if (utfByteLen === 16) {
          functionBody = context.readUtf16FromMemory(start, len);
        } else {
          functionBody = context.readUtf8FromMemory(start, len);
        }
        const id = context.functions.length;
        context.functions.push(
          Function(`"use strict";return(${functionBody})`)()
        );
        return id;
      },
      js_invoke_function(
        funcHandle: number,
        parametersStart: number,
        parametersLength: number
      ) {
        const values = context.readParameters(parametersStart, parametersLength);
        
        return context.functions[funcHandle].call(
          context,
          ...values 
        );
      },
      js_invoke_function_and_return_object(
        funcHandle: number,
        parametersStart: number,
        parametersLength: number
      ) {
        const values = context.readParameters(parametersStart, parametersLength);
        const result = context.functions[funcHandle].call(
          context,
          ...values 
        );
        if(result === undefined || result === null) {
          throw new Error("js_invoke_function_and_return_object returned undefined or null while trying to return an object");
        }
        return context.storeObject(result);
      },
      js_invoke_function_and_return_bool(
        funcHandle: number,
        parametersStart: number,
        parametersLength: number
      ) {
        const values = context.readParameters(parametersStart, parametersLength);
        const result = context.functions[funcHandle].call(
          context,
          ...values
        );
        return result ? 1 : 0;
      },
      js_invoke_function_and_return_bigint(
        funcHandle: number,
        parametersStart: number,
        parametersLength: number
      ) {
        const values = context.readParameters(parametersStart, parametersLength);
        const result = context.functions[funcHandle].call(
          context,
          ...values 
        );
        return result;
      },
      js_invoke_function_and_return_string(
        funcHandle: number,
        parametersStart: number,
        parametersLength: number
      ) {
        const values = context.readParameters(parametersStart, parametersLength);
        const result = context.functions[funcHandle].call(
          context,
          ...values 
        );

        if(result === undefined || result === null) {
          throw new Error("js_invoke_function_and_return_string returned undefined or null while trying to retrieve string.");
        }
        return context.writeUtf8ToMemory(result);
      },
      js_invoke_function_and_return_array_buffer(
        funcHandle: number,
        parametersStart: number,
        parametersLength: number
      ) {
        const values = context.readParameters(parametersStart, parametersLength);
        const result = context.functions[funcHandle].call(
          context,
          ...values 
        );

        if(result === undefined || result === null) {
          throw new Error("js_invoke_function_and_return_array_buffer returned undefined or null while trying to retrieve arraybuffer.");
        }
        return context.writeArrayBufferToMemory(result);
      }
    },context];
  },

  async loadAndRunWasm(wasmURL: string) {
    const context = await this.load(wasmURL);
    (context.module!.instance.exports.main as ()=>void)();
  },

  async load(wasmURL: string) {
    const [env,context] = JsWasm.createEnvironment();
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
  const wasmScripts = document.querySelectorAll(
    "script[type='application/wasm']"
  );
  for (let i = 0; i < wasmScripts.length; i++) {
    const src = (wasmScripts[i] as HTMLSourceElement).src;
    if (src) {
      JsWasm.loadAndRunWasm(src);
    } else {
      console.error("Script tag must have 'src' property.");
    }
  }
});
