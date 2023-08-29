import init, * as instance from "../pkg/bevy_demo";

init().then((wasm) => {
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
  // instance.greet();
});

// import * as wasm "./bevy-demo";
