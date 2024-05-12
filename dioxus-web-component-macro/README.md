# dioxus-web-component-macro

Provide a proc macro to build Dioxus web component.

## Example

The macro replace the Dioxus `#[component]` macro.

```rust, ignore
use dioxus::prelude::*;
use dioxus_web_component::{web_component, DioxusWebComponent};
use wasm_bindgen::prelude::*;

#[web_component]
fn MyWebComponent(
  attr: Option<String>,
  event: EventHandler<i64>,
) -> Element {
  rsx ! {
    div {
      // ...
    }
  }
}

#[wasm_bindgen(main)]
pub fn main() {
  // Register the web component (aka custom element)
  register_my_web_component();
}
```

```html
<!-- include the script generated with wasm-pack -->
<script type="module" src="my-web-component.js"></script>

<!-- in the body -->
<my-web-component attr="plop"></my-web-component>
```

## Usage

### Tag

The custom element tag is by built from the component name.

By default the tag is the kebab-case version of the name.
For example `MyWebComponent` becomes `my-web-component`.

You can change the default behavior with the `tag` attribute.


```rust, ignore
use dioxus::prelude::*;
use dioxus_web_component::{web_component, DioxusWebComponent};

#[web_component(tag = "plop-component")]
fn MyWebComponent(
  // ...
) -> Element { todo!() }
```


```html
<!-- in the body -->
<plop-component></plop-component>
```

⚠️ WARNING: the custom element tag name have constraints.
Currently, the macro does NOT check the validity of the tag.
See [MDN - Valid custom element names](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define#valid_custom_element_names)

### Style

You can provide the web component style with the `style` attribute.

```rust, ignore
use dioxus::prelude::*;
use dioxus_web_component::{web_component, InjectedStyle};

#[web_component(
  tag = "plop-greeting", 
  style = InjectedStyle::css(include_str!("./style.css"))
)]
fn Greeting(
  // ...
) -> Element {
  todo!()
}
```

The `dioxus_web_component::InjectedStyle` could be raw CSS included in
an HTML `<style>...</style>` element, or a link to an external stylesheet,
or a list of `InjectedStyle` style.

⚠️ WARNING: the web component is wrapped into an HTML `div` with the `dioxus` CSS class.

### Component parameters

Every parameter of your component should be either an attribute or an event.
(properties are not yet supported)

The proc macro try to detect the kind of parameter by looking at its type.
If the type start by `EventHandler` it is expected to be an event.
But, this kind of detection is not reliable, so you might need to add an annotation
to correct this behavior.

#### Attributes

Attributes are like the `href` of an `<a>` HTML element.

You can enforce the parameter to be an attribute with the `#[attribute]` annotation.

When the attribute value changed the dioxus component will be rendered.


##### Attribute `name`

The attribute name is by default the kebab-case of the parameter name.
You can choose another name with `#[attribute(name = "my-custom-name")]`.

##### Attribute `option`

The attribute could be optional or not.
The proc macro try to detect it automatically with the type name.
But the detection is not reliable, so you can use the `#[attribute(option = true)]`
to fix the detection if necessary.


##### Attribute `initial`

Attributes require to have an initial value.
This value is used when no HTML attribute is provided, or if the attribute is removed.

By default, we expect the attribute type to implement [`std::default::Default`].
If it's not the case, or if you want to use another value for your attribute you
can provide your own default expression with `#[attribute(initial = String::from("World"))]`.

##### Attribute `parse`

HTML attributes are strings and optionals, so we need to convert the attribute value
into the component parameter type.

The proc macro use the `std::str::parse` method. That's meaning the target type
need to implement the `std::str::FromStr` trait.

In case of error, the initial value (see below) is used.

If you want to change this behavior, you can provide your own parsing expression.

If the parameter type is optional, the parse expression is used in this code:
`let value = new_value.and_then(#parse);`.
If the type is NOT optional, the code looks like `let value = new_value.and_then(#parse).unwrap_or_else(|| #initial);`.

The expected type for the parsing expression is `FnOnce(String) -> Option<T>`.
The default expression is `|value| value.parse().ok()`.

For example if you have a parameter `required` of type `bool` and you want the value to be `true`
if the attribute is present whatever the content of the attribute, you could use `#[attribute(parse = |s| !s.is_empty() )]`.

#### Events

The web component could send [custom event].
If the type of the component parameter is `EventHandler`, the parameter is detected as an event.
Because this detection is not reliable, you could enforce a parameter to be
an event with the `#[event]` annotation.

The custom event detail is corresponding to the generic type of the Dioxus `EventHandler`.

⚠️ IMPORTANT: The event type need to implement `Into<JsValue>` and be `'static` (does not have any reference).

You may need implement manually.
You could use [`serde-wasm-bindgen`], [`gloo_utils::format::JsValueSerdeExt`], [`wasm_bindgen::UnwrapThrowExt`]
to implement the `Into<JsValue>` trait.


##### Event `name`

The HTML event name is detected from the parameter name by removing the `on_` (or `on`) prefix
and convert the name to kebab-case.
You can choose your own value with the `name` attribute like `#[event(name = "build")]`
to dispatch a `build` event.

##### Event `no_bubble`

By default, the event bubbles up through the DOM.
You can avoid the bubbling with `#[event(no_bubble = true)]`.

##### Event `no_cancel`

By default, the event is cancelable.
You can avoid the bubbling with `#[event(no_cancel = true)]`.


#### Properties

Not yet supported


[custom event]: https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent
[`serde-wasm-bindgen`]: https://docs.rs/serde-wasm-bindgen
[`gloo_utils::format::JsValueSerdeExt`]: https://docs.rs/gloo-utils/latest/gloo_utils/format/trait.JsValueSerdeExt.html
[`wasm_bindgen::UnwrapThrowExt`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/trait.UnwrapThrowExt.html
