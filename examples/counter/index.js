import start from "./pkg/counter.js";

// Register the web component
await start();

for (const btnSet of document.querySelectorAll("button.set")) {
	btnSet.onclick = () => {
		const label = prompt(`What's the counter label?`);
		if (label) {
			const elt = btnSet.parentElement.querySelector("plop-counter");
			elt.label = label;
		}
	};
}

for (const btnGet of document.querySelectorAll("button.get")) {
	btnGet.onclick = () => {
		const elt = btnGet.parentElement.querySelector("plop-counter");
		elt.label.then((label) => alert(`Counter label: ${label}`));
	};
}

document.querySelectorAll("plop-counter").forEach((el, index) => {
	el.addEventListener("count", (evt) => {
		console.log(`plop-counter #${index}`, evt.detail);
	});
});
