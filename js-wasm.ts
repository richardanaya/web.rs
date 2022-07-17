class Index {
  index: number;
  generation: number;

  constructor(index: number, generation: number) {
    this.index = index;
    this.generation = generation;
  }

  toNum() {
    return Number((BigInt(this.generation) << BigInt(32)) | BigInt(this.index));
  }

  static fromNum(n: number) {
    const i = Number(
      ((BigInt(n) & BigInt(0xffffffff00000000)) >> BigInt(32)) &
        BigInt(0xffffffff)
    );
    const g = n & 0xffffffff;
    return new Index(g, i);
  }
}

interface GenerationItem<T> {
  generation?: number;
  value?: T;
  nextFree?: number;
}

class GenerationalArena<T> {
  items: GenerationItem<T>[];
  generation: number;
  length: number;
  free_list_head?: number;

  constructor() {
    this.items = [];
    this.generation = 0;
    this.free_list_head = undefined;
    this.length = 0;
  }

  insert(v: T) {
    // lets use the first free entry if we have one
    if (this.free_list_head !== undefined) {
      let i = this.free_list_head;
      this.free_list_head = this.items[i].nextFree;
      this.items[i] = {
        generation: this.generation,
        value: v,
      };
      this.length += 1;
      return new Index(i, this.generation);
    }

    this.items.push({
      generation: this.generation,
      value: v,
    });
    const idx = new Index(this.items.length - 1, this.generation);
    this.length += 1;
    return idx;
  }

  contains(idx: Index) {
    return this.get(idx) !== undefined;
  }

  get(i: Index) {
    let e = this.items[i.index];
    if (e && e.generation === i.generation) {
      return e.value;
    }
    return undefined;
  }

  remove(idx: Index) {
    if (idx.index >= this.items.length) {
      return undefined;
    }

    let e = this.items[idx.index];
    if (e.generation !== undefined && e.generation == idx.generation) {
      this.generation += 1;
      this.items[idx.index] = {
        nextFree: this.free_list_head,
      };
      this.free_list_head = idx.index;
      this.length -= 1;
      return e.value;
    }
    return undefined;
  }

  *[Symbol.iterator]() {
    for (let i = 0; i < this.items.length; i++) {
      const x = this.items[i];
      if (x.generation !== undefined) {
        yield { index: new Index(i, x.generation), value: x.value };
      }
    }
  }

  indices() {
    return {
      items: this.items,
      [Symbol.iterator]: function* iter() {
        for (let i = 0; i < this.items.length; i++) {
          const x = this.items[i];
          if (x.generation !== undefined) {
            yield new Index(i, x.generation);
          }
        }
      },
    };
  }

  values() {
    return {
      items: this.items,
      [Symbol.iterator]: function* iter() {
        for (let i = 0; i < this.items.length; i++) {
          const x = this.items[i];
          if (x.generation !== undefined) {
            yield x.value;
          }
        }
      },
    };
  }
}

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
  objects: GenerationalArena<unknown>;
  utf8dec: TextDecoder;
  utf8enc: TextEncoder;
  utf16dec: TextDecoder;
  toCallbackArg: (arg: number | object) => number;
  storeObject: (obj: any) => number;
  releaseObject: (objHandle: number) => void;
  module?: WebAssembly.WebAssemblyInstantiatedSource;
  readUtf8FromMemory: (start: number, end: number) => string;
  readUtf16FromMemory: (start: number, end: number) => string;
  getMemory: () => Uint8Array;
  createAllocation: (size: number) => [number,number] ;
  createCallback: (cb: number) => () => void;
  writeUtf8ToMemory: (txt: string) => number;
  readUint8ArrayFromMemory: (start: number) => Uint8Array;
  getObject: (handle: number) => any;
}

const JsWasm = {
  createEnvironment(): [WebAssembly.ModuleImports, JSWasmHandlerContext] {
    const arena = new GenerationalArena();
    arena.insert(undefined);
    arena.insert(null);
    arena.insert(self);
    arena.insert(typeof document != "undefined" ? document : null);
    arena.insert(typeof document != "undefined" ? document.body : null);
    const context: JSWasmHandlerContext = {
      functions: [
        function () {
          debugger;
          return 0;
        },
      ],
      objects: arena,
      utf8dec: new TextDecoder("utf-8"),
      utf8enc: new TextEncoder(),
      utf16dec: new TextDecoder("utf-16"),
      toCallbackArg: function (arg: number | object) {
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
          cb: number,
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
      storeObject: function (obj: unknown) {
        const index = this.objects.insert(obj);
        return index.toNum();
      },
      getObject: function (handle: number) {
        return this.objects.get(Index.fromNum(handle));
      },
      releaseObject: function (handle: number) {
        this.objects.remove(Index.fromNum(handle));
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
      js_release(obj: number) {
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
