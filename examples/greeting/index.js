import start from "./pkg/greeting.js";

// Register the web component
await start();

const btn = document.querySelector("button");
btn.onclick = () => {
  const name = prompt(`What's your name?`);
  if (name) {
    document
      .querySelectorAll("plop-greeting")
      .forEach((el) => el.setAttribute("name", name));
  }
};

const colorInput = document.querySelector("input[type=color]");
colorInput.oninput = () => {
  let color = colorInput.value;
    document
      .querySelectorAll("plop-greeting")
      .forEach((el) => el.style.setProperty("--my-color", color));
};
