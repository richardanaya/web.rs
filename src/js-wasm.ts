import { ExternRef } from 'externref_polyfill';

interface JSWasmHandlerContext {
  functions: ((
    a: number,
    b: number,
    c: number,
    d: number,
    e: number,
    f: number,
    g: number,
    h: number,
    i: number,
    j: number
  ) => number)[];
  utf8dec: TextDecoder;
  utf8enc: TextEncoder;
  utf16dec: TextDecoder;
  toCallbackArg: (arg: number | object) => number | bigint;
  storeObject: (obj: unknown) => bigint;
  releaseObject: (objHandle: bigint) => void;
  module?: WebAssembly.WebAssemblyInstantiatedSource;
  readUtf8FromMemory: (start: number, end: number) => string;
  readUtf16FromMemory: (start: number, end: number) => string;
  getMemory: () => Uint8Array;
  createAllocation: (size: number) => [number,number] ;
  createCallback: (cb: number) => () => void;
  writeUtf8ToMemory: (txt: string) => number;
  readUint8ArrayFromMemory: (start: number) => Uint8Array;
  getObject: (handle: bigint) => unknown;
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
      toCallbackArg: function (arg: number | object) : number | bigint {
        if (typeof arg === "object") {
          return context.storeObject(arg);
        }
        return arg;
      },
      createCallback: function (cb: number) {
        if (!this.module) {
          throw new Error("module not set");
        }
        const fnHandleCallback = this.module.instance.exports.handle_callback as (
          cb: number ,
          a: number | bigint,
          b: number | bigint,
          c: number | bigint,
          d: number | bigint,
          e: number | bigint,
          f: number | bigint,
          g: number | bigint,
          h: number | bigint,
          i: number | bigint,
          j: number | bigint
        ) => void;
        return function () {
          const arg = arguments;
          fnHandleCallback(
            cb,
            context.toCallbackArg(arg[0]),
            context.toCallbackArg(arg[1]),
            context.toCallbackArg(arg[2]),
            context.toCallbackArg(arg[3]),
            context.toCallbackArg(arg[4]),
            context.toCallbackArg(arg[5]),
            context.toCallbackArg(arg[6]),
            context.toCallbackArg(arg[7]),
            context.toCallbackArg(arg[8]),
            context.toCallbackArg(arg[9])
          );
        };
      },
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
      readUtf16FromMemory: function (start: number, len: number) {
        const text = this.utf16dec.decode(
          this.getMemory().subarray(start, start + len)
        );
        return text;
      },
      readUint8ArrayFromMemory(start: number) {
        if (!this.module) {
          throw new Error("module not set");
        }
        const data32 = new Uint32Array(
          (this.module.instance.exports.memory as WebAssembly.Memory).buffer
        );
        const ptr = data32[start / 4];
        const length = data32[ptr / 4];
        let b = this.getMemory().slice(ptr + 4, ptr + 4 + length);
        return new Uint8Array(b);
      },
      storeObject: function (obj: unknown) : bigint {
        return ExternRef.create(obj);
      },
      getObject: function (handle: bigint) {
        return ExternRef.load(handle);
      },
      releaseObject: function (handle: bigint) {
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
    };
    return [{
      abort() {
        throw new Error("WebAssembly module aborted");
      },
      js_release(obj: bigint) {
        context.releaseObject(obj);
      },
      js_register_function(start: number, len: number, utfByteLen: number) {
        let functionBody;
        if (utfByteLen === 16) {
          functionBody = context.readUtf16FromMemory(start, len);
        } else {
          functionBody = context.readUtf8FromMemory(start, len);
        }
        let id = context.functions.length;
        context.functions.push(
          Function(`"use strict";return(${functionBody})`)()
        );
        return id;
      },
      js_invoke_function(
        funcHandle: number,
        a: number,
        b: number,
        c: number,
        d: number,
        e: number,
        f: number,
        g: number,
        h: number,
        i: number,
        j: number
      ) {
        return context.functions[funcHandle].call(
          context,
          a,
          b,
          c,
          d,
          e,
          f,
          g,
          h,
          i,
          j
        );
      },
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

if ((window as any).WasmScriptComponents) {
  (window as any).WasmScriptComponents["js-wasm"] = function (e: any) {
    return {
      ...e,
      ...JsWasm.createEnvironment(),
    };
  };
}
