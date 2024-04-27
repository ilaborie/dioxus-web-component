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
