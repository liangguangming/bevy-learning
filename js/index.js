import init from "../pkg/bevy_demo";

init().then((instance) => {
  console.log("init success!!!", instance);
  try {
    instance.main();
  } catch (error) {
    if (error.message && error.message.indexOf("Using exceptions for control flow, don't mind me.") < 0) {
      throw error;
    }
  }
  resizeCanvas();
})

// import * as wasm "./bevy-demo";
