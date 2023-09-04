import init, * as instance from "../pkg/bevy_demo";
import { resizeCanvas } from "./event";

export class BevyApp {
  constructor() {
    this.isReady;
    this.initApp();
    this.messageCallbackMap = new Map();
    this.wasm = null;
  }

  initApp() {
    this.isReady = new Promise((resolve) => {
      init().then((wasm) => {
        this.wasm = wasm;
        // 注册接收 wasm 信息的回调
        instance.on_message((msg) => {
          this.messageCallbackMap.forEach((value, cb) => {
            cb(msg);
          })
        });
        try {
          wasm.wasm_main();
        } catch (error) {
          if (error.message && error.message.indexOf("Using exceptions for control flow, don't mind me.") < 0) {
            throw error;
          }
        }
        resizeCanvas();
        resolve();
      });
    });
  }

  onMessage(cb) {
    if (this.messageCallbackMap.has(cb)) {
      return;
    }
    this.messageCallbackMap.set(cb, true);
  }

  removeMessageCallback(cb) {
    this.messageCallbackMap.delete(cb);
  }

  sendMessageToWasm(msg) {
    if (!msg) {
      return;
    }
    instance.send_msg_to_wasm(msg);
  }

  /**
   * 写入 buffer
   * @param {String} key
   * @param {Uint8Array} buffer 
   */
  writeBuffer(key, buffer) {
    const len = buffer.byteLength - buffer.byteOffset;
    const ptr  = instance.new_buffer(key, len);
    const uint8Array = new Uint8Array(this.wasm.memory.buffer, ptr, len);
    uint8Array.set(buffer);
  }

  /**
   * 读取 buffer, 注意： 不要修改【从安全的角度，可以 copy 一份给 js】
   * @param {String} key
   * @returns 
   */
  readBuffer(key) {
    const meta = instance.get_buffer_meta(key);
    if (!meta) {
      return;
    }

    const { ptr, len } = meta;
    const buffer = new Uint8Array(this.wasm.memory.buffer, ptr, len);

    return buffer;
  }

  deleteBuffer(filename) {
    instance.remove_buffer(filename)
  }
}

export default new BevyApp();