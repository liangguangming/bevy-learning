import app from "./bevyApp.js";

async function main() {
  app.onMessage((str) => {
    console.log(str);
  });
  await app.isReady;
  app.sendMessageToWasm("Hi wasm, I am from web");
}

main();
