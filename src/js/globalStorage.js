const heap = new Array();
let heap_next = 0;
let wasm;

/**
* 往 js 堆中添加对象
* @param {any} obj 要存储的对象
* @returns 索引
*/
export function addHeapObject(obj) {
  if (heap_next === heap.length) {
    heap.push(heap.length + 1);
  }
  const idx = heap_next;
  heap_next = heap[idx];
  heap[idx] = obj;
  return idx;
}

export function addHeapObjectByBufferMeta(bufferMeta) {
  if (!wasm) {
    throw new Error("not found wasm instance");
  }

  const { ptr, len } = bufferMeta;
  const uint8array = new Uint8Array(len);

  const wasmMemory = new Uint8Array(wasm.memory.buffer, ptr, len);

  uint8array.set(wasmMemory);
  const idx = addHeapObject(uint8array);

  return idx;
}

export function getHeapObject(idx) {
  return heap[idx];
}

export function getHeapObjectToVal(idx) {
  const uint8Array = heap[idx];

  const ptr = wasm.__wbindgen_malloc(uint8Array.byteLength, 1);

  return {
    ptr,
    len: uint8Array.byteLength
  }
}

/**
* 移除堆对象
* @param {number} idx 索引
*/
export function dropHeapObject(idx) {
 heap[idx] = heap_next;
 heap_next = idx;
}

export function setWasm(w) {
  wasm = w;
}