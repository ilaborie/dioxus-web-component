import start from "./pkg/counter.js";

// Register the web component
await start();

document.querySelectorAll("plop-counter").forEach((el, index) => {
  el.addEventListener("count", (evt) => {
    console.log(`plop-counter #${index}`, evt.detail);
  });
});
