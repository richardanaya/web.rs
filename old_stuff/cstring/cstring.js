const utf8dec = new TextDecoder("utf-8");
const utf8enc = new TextEncoder("utf-8");

export function extractCString(mem, ptr) {
  const memory = new Uint8Array(mem);
  const str = [];
  let i = ptr;
  while (memory[i] !== 0) {
    str.push(memory[i]);
    i++;
  }
  return utf8dec.decode(new Uint8Array(str));
}

export function insertString(getMemory, malloc, str) {
  const bytes = utf8enc.encode(str);
  const len = bytes.length;
  const start = malloc(len);
  const memory = new Uint8Array(getMemory());
  memory.set(bytes, start);
  return [start, len];
}
