import init, * as instance from "../pkg/bevy_demo";
import { resizeCanvas } from "./event";

export class BevyApp {
  constructor() {
    this.isReady;
    this.initApp();
    this.messageCallbackMap = new Map();
  }

  initApp() {
    this.isReady = new Promise((resolve) => {
      init().then((wasm) => {
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
}

export default new BevyApp();