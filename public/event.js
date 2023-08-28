const canvas = document.getElementById("xspiral");

const padding = 16;

function resizeCanvas() {
  const win = window;

  const width = win.innerWidth - 16;
  const height = win.innerHeight - 16;

  console.log("width: ", width, "height: ", height);

  canvas.clientWidth = width;
  canvas.clientHeight = height;
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;
  console.log("canvas: ", canvas.style.width, canvas.style.height);
}

window.addEventListener("resize", () => {
  resizeCanvas();
});
