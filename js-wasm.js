async function load_and_run_wasm(wasmURL) {
  let context = {
    functions: [],
    free_locations: [],
    objects: [
      undefined,
      null,
      self,
      typeof document != "undefined" ? document : null
    ],
    utf8dec: new TextDecoder("utf-8"),
    utf8enc: new TextEncoder("utf-8"),
    getCStringFromMemory: function (start) {
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
    getUtf8FromMemory: function (start, len) {
      let memory = new Uint8Array(this.module.instance.exports.memory.buffer);
      let text = this.utf8dec.decode(memory.subarray(start, start + len));
      return text;
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
  let response = await fetch(wasmURL);
  let bytes = await response.arrayBuffer();
  let module = await WebAssembly.instantiate(bytes, {
    env: {
      js_register_function(start, len) {
        let functionBody = context.getUtf8FromMemory(start, len);
        let id = context.functions.length;
        context.functions.push(eval("(" + functionBody + ")").bind(context));
        return id;
      },
      js_invoke_function(funcHandle, a, b, c, d, e, f, g, h, i, j) {
        return context.functions[funcHandle](
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
  });
  context.module = module;
  let num = module.instance.exports.main();
}

document.addEventListener("DOMContentLoaded", function () {
  let wasmScripts = document.querySelectorAll("script[type='application/wasm']");
  for (let i = 0; i < wasmScripts.length; i++) {
    let src = wasmScripts[i].src;
    if (src) {
      load_and_run_wasm(src);
    } else {
      console.error("Script tag must have 'src' property.");
    }
  }
});