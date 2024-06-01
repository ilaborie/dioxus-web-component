# Counter example

This example provide a `plop-counter` web component that 
emit a `count` event when the counter is updated.
(open your browser console to see event logged)

The component also have a `label` property that can be set with Javascript.

To build this sample, use [wasm-pack]

```shell
wasm-pack build --release --target web
```

See [index.html](index.html) and [index.js](index.js) to see how to use it.

[wasm-pack]: https://github.com/rustwasm/wasm-pack
