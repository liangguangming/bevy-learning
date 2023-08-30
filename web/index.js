import init, * as instance from "../pkg/bevy_demo";
import { resizeCanvas } from "./event";

init().then((wasm) => {
  // 注册接收 wasm 信息的回调
  instance.on_message((msg) => {
    console.log("cb -- receive msg from wasm: ", msg);
  });
  try {
    wasm.wasm_main();
  } catch (error) {
    if (error.message && error.message.indexOf("Using exceptions for control flow, don't mind me.") < 0) {
      throw error;
    }
  }
  resizeCanvas();
}).then(() => {
  console.log("finish wasm_main");
  console.log("1 + 2 = ", instance.add(1, 2));
  instance.send_msg_to_wasm("hello rust");
  const xspiralObj = new instance.StructObj("Xspiral", 23);
  console.log("xspiralObj: ", xspiralObj.get_name(), xspiralObj.get_value());

  xspiralObj.set_name("spiral");

  console.log("xspiralObj: ", xspiralObj.get_name(), xspiralObj.get_value());
  xspiralObj.free();
})

// import * as wasm "./bevy-demo";
