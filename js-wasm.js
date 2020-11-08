window.JsWasm = {
  createEnvironment() {
    let context = {
      functions: [],
      free_locations: [],
      objects: [
        undefined,
        null,
        self,
        typeof document != "undefined" ? document : null,
        typeof document != "undefined" ? document.body : null
      ],
      utf8dec: new TextDecoder("utf-8"),
      utf8enc: new TextEncoder("utf-8"),
      createCallback: function (cb) {
        let fnHandleCallback = this.module.instance.exports.handle_callback;
        return function () {
          const args = arguments;
          fnHandleCallback(cb, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9]);
        }
      },
      readCStringFromMemory: function (start) {
        const data = new Uint8Array(this.module.instance.exports.memory.buffer);
        const str = [];
        let i = start;
        while (data[i] !== 0) {
          str.push(data[i]);
          i++;
        }
        return this.utf8dec.decode(new Uint8Array(str));
      },
      writeCStringToMemory(str, start) {
        let bytes = this.utf8enc.encode(str + String.fromCharCode(0));
        let len = bytes.length;
        const memory = new Uint8Array(this.module.instance.exports.memory.buffer);
        memory.set(bytes, start);
      },
      readUtf8FromMemory: function (start, len) {
        let memory = new Uint8Array(this.module.instance.exports.memory.buffer);
        let text = this.utf8dec.decode(memory.subarray(start, start + len));
        return text;
      },
      readUint8ArrayFromMemory(start) {
        const data32 = new Uint32Array(this.module.instance.exports.memory.buffer);
        const ptr = data32[start / 4];
        const length = data32[ptr / 4];
        let b = mem.slice(ptr + 4, ptr + 4 + length);
        return new Uint8Array(b);
      },
      writeUtf8ToMemory: function (start, str) {
        let bytes = utf8enc.encode(str);
        let len = bytes.length;
        const memory = new Uint8Array(this.module.instance.exports.memory.buffer);
        memory.set(bytes, start);
      },
      storeObject: function (obj) {
        let handle = this.objects.length;
        if (this.free_locations.length > 0) {
          handle = this.free_locations.pop();
        }
        this.objects.push(obj);
        return handle;
      },
      getObject: function (handle) {
        return this.objects[handle];
      },
      releaseObject: function (handle) {
        this.objects[handle] = null;
        this.free_locations.push(handle);
      }
    };
    return {
      context,
      js_register_function(start, len) {
        let functionBody = context.readUtf8FromMemory(start, len);
        let id = context.functions.length;
        context.functions.push(eval("(" + functionBody + ")"));
        return id;
      },
      js_invoke_function(funcHandle, a, b, c, d, e, f, g, h, i, j) {
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
      }
    }
  },

  async load_and_run_wasm(wasmURL) {
    const env = JsWasm.createEnvironment();
    const response = await fetch(wasmURL);
    const bytes = await response.arrayBuffer();
    const module = await WebAssembly.instantiate(bytes, {
      env
    });
    env.context.module = module;
    module.instance.exports.main();
  }
}

document.addEventListener("DOMContentLoaded", function () {
  const wasmScripts = document.querySelectorAll("script[type='application/wasm']");
  for (let i = 0; i < wasmScripts.length; i++) {
    const src = wasmScripts[i].src;
    if (src) {
      JsWasm.load_and_run_wasm(src);
    } else {
      console.error("Script tag must have 'src' property.");
    }
  }
});

if (window.WasmScriptComponents) {
  window.WasmScriptComponents["js-wasm"] = function (e) {
    return {
      ...e,
      ...JsWasm.createEnvironment()
    }
  }
}