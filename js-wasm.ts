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
    let i = Number(
      ((BigInt(n) & BigInt(0xffffffff00000000)) >> BigInt(32)) &
        BigInt(0xffffffff)
    );
    let g = n & 0xffffffff;
    return new Index(g, i);
  }
}

interface GenerationItem {
  generation?: number;
  value?: any;
  next_free?: number;
}

class GenerationalArena {
  items: GenerationItem[];
  generation: number;
  length: number;
  free_list_head?: number;

  constructor() {
    this.items = [];
    this.generation = 0;
    this.free_list_head = undefined;
    this.length = 0;
  }

  insert(v: any) {
    // lets use the first free entry if we have one
    if (this.free_list_head !== undefined) {
      let i = this.free_list_head;
      this.free_list_head = this.items[i].next_free;
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
    let idx = new Index(this.items.length - 1, this.generation);
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
        next_free: this.free_list_head,
      };
      this.free_list_head = idx.index;
      this.length -= 1;
      return e.value;
    }
    return undefined;
  }

  *[Symbol.iterator]() {
    for (var i = 0; i < this.items.length; i++) {
      let x = this.items[i];
      if (x.generation !== undefined) {
        yield { index: new Index(i, x.generation), value: x.value };
      }
    }
  }

  indices() {
    return {
      items: this.items,
      [Symbol.iterator]: function* iter() {
        for (var i = 0; i < this.items.length; i++) {
          let x = this.items[i];
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
        for (var i = 0; i < this.items.length; i++) {
          let x = this.items[i];
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
  objects: GenerationalArena;
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
  malloc: (size: number) => number;
  createCallback: (cb: number) => () => void;
  readCStringFromMemory: (start: number) => string;
  writeCStringToMemory: (txt: string) => number;
  writeUtf8ToMemory: (txt: string) => number;
  readUint8ArrayFromMemory: (start: number) => Uint8Array;
  getObject: (handle: number) => any;
}

(window as any).JsWasm = {
  createEnvironment() {
    let arena = new GenerationalArena();
    arena.insert(undefined);
    arena.insert(null);
    arena.insert(self);
    arena.insert(typeof document != "undefined" ? document : null);
    arena.insert(typeof document != "undefined" ? document.body : null);
    let context: JSWasmHandlerContext = {
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
        let fnHandleCallback = this.module.instance.exports.handle_callback as (
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
      readCStringFromMemory: function (start: number) {
        const data = this.getMemory();
        const str = [];
        let i = start;
        while (data[i] !== 0) {
          str.push(data[i]);
          i++;
        }
        return this.utf8dec.decode(new Uint8Array(str));
      },
      writeCStringToMemory(str: string) {
        const bytes = this.utf8enc.encode(str + String.fromCharCode(0));
        const len = bytes.length;
        const start = this.malloc(len);
        this.getMemory().set(bytes, start);
        return start;
      },
      readUtf8FromMemory: function (start: number, len: number) {
        const text = this.utf8dec.decode(
          this.getMemory().subarray(start, start + len)
        );
        return text;
      },
      malloc: function (size: number): number {
        if (!this.module) {
          throw new Error("module not set");
        }
        return (this.module.instance.exports.malloc as (
          size: number
        ) => number)(size);
      },
      writeUtf8ToMemory: function (str: string) {
        const bytes = this.utf8enc.encode(str);
        const len = bytes.length;
        const start = this.malloc(len);
        this.getMemory().set(bytes, start);
        return start;
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
      storeObject: function (obj: any) {
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
    return {
      context,
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
    };
  },

  async load_and_run_wasm(wasmURL: string) {
    const env = (window as any).JsWasm.createEnvironment();
    const response = await fetch(wasmURL);
    const bytes = await response.arrayBuffer();
    const module = await WebAssembly.instantiate(bytes, {
      env,
    });
    env.context.module = module;
    (module.instance.exports as any).main();
  },
};

document.addEventListener("DOMContentLoaded", function () {
  const wasmScripts = document.querySelectorAll(
    "script[type='application/wasm']"
  );
  for (let i = 0; i < wasmScripts.length; i++) {
    const src = (wasmScripts[i] as HTMLSourceElement).src;
    if (src) {
      (window as any).JsWasm.load_and_run_wasm(src);
    } else {
      console.error("Script tag must have 'src' property.");
    }
  }
});

if ((window as any).WasmScriptComponents) {
  (window as any).WasmScriptComponents["js-wasm"] = function (e: any) {
    return {
      ...e,
      ...(window as any).JsWasm.createEnvironment(),
    };
  };
}
