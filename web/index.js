import app from "./bevyApp.js";

async function main() {
  app.onMessage((str) => {
    console.log(str);
  });
  await app.isReady;
  app.sendMessageToWasm("Hi wasm, I am from web");

  const buf = new Uint8Array([1, 2, 3, 4, 5]);

  const filename = "xspiral.jpg";
  app.writeBuffer(filename, buf);

  const wasmBuffer = app.readBuffer(filename);

  console.log("wasmbuffer: ", wasmBuffer);
}

main();
