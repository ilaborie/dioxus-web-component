import start from "./pkg/counter.js";

// Register the web component
await start();

document.querySelectorAll("button.set").forEach( btnSet => {
  btnSet.onclick = () => {
    const label = prompt(`What's the counter label?`);
    if (label) {
      const elt = btnSet.parentElement.querySelector("plop-counter");
      elt.label = label;
    }
  };
});

document.querySelectorAll("button.get").forEach(btnGet => {
  btnGet.onclick = () => {
      const elt = btnGet.parentElement.querySelector("plop-counter");
      elt.label.then(label => alert(`Counter label: ${label}`));
  };
});


document.querySelectorAll("plop-counter").forEach((el, index) => {
  el.addEventListener("count", (evt) => {
    console.log(`plop-counter #${index}`, evt.detail);
  });
});
