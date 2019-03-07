import { Plasma } from "plasma-wasm-rust";
import { memory } from "plasma-wasm-rust/plasma_wasm_rust_bg";

const plasma = Plasma.new();
const width = plasma.width();
const height = plasma.height();

const canvas = document.getElementById("plasmaCanvas");
canvas.height = height;
canvas.width = width;

const ctx = canvas.getContext("2d");

const renderLoop = () => {
    plasma.tick();
    drawPlasma();
    requestAnimationFrame(renderLoop);
}

const drawPlasma = () => {
    const memoryPtr = plasma.buffer();
    const buffer = new Uint8ClampedArray(memory.buffer, memoryPtr, width * height * 4);
    const imageData = new ImageData(buffer, width, height);
    ctx.putImageData(imageData, 0, 0);
}

renderLoop();