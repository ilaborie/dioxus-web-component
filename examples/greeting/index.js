import start from "./pkg/greeting.js";

// Register the web component
await start();

// Register the change name button
const btn = document.querySelector("button");
btn.onclick = () => {
	const name = prompt(`What's your name?`);
	if (name) {
		for (const el of document.querySelectorAll("plop-greeting")) {
			// Set with attribute
			el.setAttribute("name", name);
			// Or directly with property
			el.name = name;
		}
	}
};

// Register the change color button
const colorInput = document.querySelector("input[type=color]");
colorInput.oninput = () => {
	const color = colorInput.value;
	for (const el of document.querySelectorAll("plop-greeting")) {
		el.style.setProperty("--my-color", color);
	}
};
