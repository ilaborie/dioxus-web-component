# dioxus-web-component-macro

Provide a proc macro to build Dioxus web component.

## Example

The macro replaces the Dioxus `#[component]` macro.

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

The custom element tag is built from the component name.

By default, the tag is the kebab-case version of the name.
For example, `MyWebComponent` becomes `my-web-component`.

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

ℹ️ INFO: the custom element tag name has constraints.
The macro checks the validity of the tag for you.
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
or a list of `InjectedStyle` styles.

⚠️ WARNING: the web component is wrapped into an HTML `div` with the `dioxus` CSS class.

### Component parameters

Every parameter of your component should be an attribute, a property, or an event.
Note that a parameter could be both an attribute and a property.

The proc macro tries to detect the kind of parameter by looking at its type.
If the type starts by `EventHandler` it is expected to be an event.
But, this kind of detection is not reliable, so you might need to add an annotation
to correct this behavior.

The annotations are also required if you need to customize the behavior.

#### Attributes

Attributes are like the `href` of an `<a>` HTML element.

You can enforce the parameter to be an attribute with the `#[attribute]` annotation.

When the attribute value changes the dioxus component will be rendered.

The HTML value of an attribute is a `String`, so you should be able
to parse that string into the target type.

##### Attribute `name`

The attribute name is by default the kebab-case of the parameter name.
You can choose another name with `#[attribute(name = "my-custom-name")]`.

##### Attribute `option`

The attribute could be optional or not.
The proc macro tries to detect it automatically with the type name.
However the detection is not reliable, so you can use the `#[attribute(option = true)]`
to fix the detection if necessary.


##### Attribute `initial`

Attributes require to have an initial value.
This value is used when no HTML attribute is provided, or if the attribute is removed.

By default, we expect the attribute type to implement [`std::default::Default`].
If it's not the case, or if you want to use another value for your attribute you
can provide your default expression with `#[attribute(initial = String::from("World"))]`.

Note that `Option<T>` implements `Default` with the `None` value
even if `T` does not implement itself `Default`.

##### Attribute `parse`

HTML attributes are strings and optional, so we need to convert the attribute value
into the component parameter type.

The proc macro uses the `std::str::parse` method. That means the target type
needs to implement the `std::str::FromStr` trait.

In case of an error, the initial value (see below) is used.

If you want to change this behavior, you can provide your parsing expression.

If the parameter type is optional, the parse expression is used in this code:
`let value = new_value.and_then(#parse);`.
If the type is NOT optional, the code looks like `let value = new_value.and_then(#parse).unwrap_or_else(|| #initial);`.

The expected type for the parsing expression is `FnOnce(String) -> Option<T>`.
The default expression is `|value| value.parse().ok()`.

For example, if you have a parameter `required` of type `bool` and you want the value to be `true`
if the attribute is present whatever the content of the attribute, you could use `#[attribute(parse = |s| !s.is_empty() )]`.

#### Property

On the Rust side of the code, properties work like attributes.
The property is not accessible with pure HTML,
you need Javascript to get/set the property.

Instead of the `String` representation, you need to be able to convert the Rust type into a 
Javascript type (here a `wasm_bindgen::JsValue`).
For the setter, you need the opposite conversion.

##### Attribute `name`

The attribute name is by default the camelCase of the parameter name.
You can choose another name with `#[property(name = "valueAsDate")]`.

##### Attribute `readonly`

If `true`, it avoids setting the property from the javascript side.
By default getter and setter are generated.

##### Attribute `initial`

Properties require to have an initial value.
This value is used when the component is initialized.

By default, we expect the property type to implement [`std::default::Default`].
If it's not the case, or if you want to use another value for your property you
can provide your default expression with `#[property(initial = String::from("World"))]`.

##### Attribute `try_into_js`

For the getter, the property value should be converted to a `wasm_bindgen::JsValue`.
By default, we use the `std::convert::TryInto` implementation.

Note that there are many ways to implement `TryInto<JsValue>`, 
for example with `impl TryFrom<T> for JsValue` or even `impl From<T> for JsValue`.
See [Rust `TryInto`](https://doc.rust-lang.org/std/convert/trait.TryInto.html).

You can provide your custom conversion into the `JsValue`
with the `try_into_js` attribute.
The expected type for the parsing expression is `FnOnce(T) -> Result<JsValue, _>`.
Not that we do not care about the error type because
the error case is ignored and returns `undefined`.

The default expression is `|value| value.try_into()`.

Example to convert a custom type that wraps a `bool`:

```rust, ignore
// ...
#[property(
    try_into_js = |prop| {
        let js_value = if prop.0 {
            JsValue::TRUE
        } else {
            JsValue::FALSE
        };
        Ok::<_, Infallible>(js_value)
    },
)]
prop2: MyProp,
// ...

#[derive(Clone, PartialEq, Default)]
struct MyProp(bool);
```

But in that situation, the recommended way is to implement `From<MyProp> for JsValue`.

##### Attribute `try_from_js`

For the setter, the property value should be converted from a `wasm_bindgen::JsValue`.
By default, we use the `std::convert::TryInto` implementation.

Note that there are many ways to implement `TryInto<T>`, 
for example with `impl TryFrom<JsValue> for T` or even `impl From<JsValue> for T`.
See [Rust `TryInto`](https://doc.rust-lang.org/std/convert/trait.TryInto.html).

You can provide your custom conversion from the `JsValue`
with the `try_from_js` attribute.
The expected type for the parsing expression is `FnOnce(JsValue) -> Result<T, _>`.
Not that we do not care about the error type because
the error case is ignored.

The default expression is `|value| value.try_into()`.

Example to convert a custom type that wraps a `bool`:

```rust, ignore
// ...
#[property(
    try_from_js= |value| Ok::<_, Infallible>(MyProp(value.is_truthy())),
)]
prop2: MyProp,
// ...

#[derive(Clone, PartialEq, Default)]
struct MyProp(bool);
```

But in that situation, the recommended way is to implement `From<JsValue> for MyProp`.

#### Events

The web component could send [custom events].
If the type of the component parameter is `EventHandler`, the parameter is detected as an event.
Because this detection is not reliable, you could enforce a parameter to be
an event with the `#[event]` annotation.

The custom event detail corresponds to the generic type of the Dioxus `EventHandler`.

⚠️ IMPORTANT: The event type needs to implement `Into<JsValue>` and be `'static` (does not have any reference).

You may need to implement it manually.
You could use [`serde-wasm-bindgen`], [`gloo_utils::format::JsValueSerdeExt`], [`wasm_bindgen::UnwrapThrowExt`]
to implement the `Into<JsValue>` trait.


##### Event `name`

The HTML event name is detected from the parameter name by removing the `on_` (or `on`) prefix
and converting the name to kebab-case.
You can choose your value with the `name` attribute like `#[event(name = "build")]`
to dispatch a `build` event.

##### Event `no_bubble`

By default, the event bubbles up through the DOM.
You can avoid the bubbling with `#[event(no_bubble = true)]`.

##### Event `no_cancel`

By default, the event is cancelable.
You can avoid the bubbling with `#[event(no_cancel = true)]`.


[custom events]: https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent
[`serde-wasm-bindgen`]: https://docs.rs/serde-wasm-bindgen
[`gloo_utils::format::JsValueSerdeExt`]: https://docs.rs/gloo-utils/latest/gloo_utils/format/trait.JsValueSerdeExt.html
[`wasm_bindgen::UnwrapThrowExt`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/trait.UnwrapThrowExt.html
