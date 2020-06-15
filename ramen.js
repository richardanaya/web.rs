async function load_and_run_wasm(wasmURL) {
  let context = {
    functions: [],
    objects: [],
    utf8dec: new TextDecoder("utf-8"),
    utf8enc: new TextEncoder("utf-8"),
    getUtf8FromMemory: function(start, len) {
      let memory = new Uint8Array(this.module.instance.exports.memory.buffer);
      let text = this.utf8dec.decode(memory.subarray(start, start + len));
      return text;
    },
    storeObject: function(obj) {
      let handle = this.objects.length;
      this.objects.push(obj);
      return handle;
    },
    getObject: function(handle) {
      return this.objects[handle];
    },
    releaseObject: function(handle) {
      this.objects[handle] = null;
    }
  };
  let response = await fetch(wasmURL);
  let bytes = await response.arrayBuffer();
  let module = await WebAssembly.instantiate(bytes, {
    env: {
      js_register_function(start, len) {
        let functionBody = context.getUtf8FromMemory(start, len);
        let id = context.functions.length;
        context.functions.push(eval("(" + functionBody + ")"));
        return id;
      },
      js_invoke_function(funcHandle, a, b, c, d, e, f, g, h, i, j) {
        return context.functions[funcHandle](
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
  });
  context.module = module;
  let num = module.instance.exports.main();
}

document.addEventListener("DOMContentLoaded", function(){
  let wasmScripts = document.querySelectorAll("script[type='application/wasm']");
  for(let i = 0;i<wasmScripts.length;i++){
    let src = wasmScripts[i].src;
    if(src){
      load_and_run_wasm(src);
    } else {
      console.error("Script tag must have 'src' property.");
    }
  }
});