# Dioxus Web Component

This crate provides a bridge to expose a [Dioxus] component as a [web component].

This crate supports web component attributes and custom events.
You can also add CSS style to your web component.

Take a look at the examples to see the usage in a full project:
<https://github.com/ilaborie/dioxus-web-component/tree/main/examples>


If you are new to WebAssembly with Rust, take a look at the [Rust WebAssembly book] first.

## Usage with macro

Ideally, you only need to replace the Dioxus `#[component]` by `#[web_component]`.
Then you should register the web component with [wasm-bindgen].
To finish, you can create the [npm] package with [wasm-pack].


```rust, ignore
use dioxus::prelude::*;
use dioxus_web_component::web_component;

#[web_component]
fn MyWebComponent(
    attribute: String,
    on_event: EventHandler<i64>,
) -> Element {
    todo!()
}

// Function to call from the JS side
#[wasm_bindgen]
pub fn register() {
  // Register the web component (aka custom element)
  register_my_web_component();
}
```

Then call the function from the JS side.


### Customization of the web component

The `#[web_component]` annotation can be configured with:

* `tag` to set the HTML custom element tag name.
  By default, it's the kebab case version of the function name.
* `style` to provide the [`InjectedStyle`] to your component.

Parameters of the component could be either an attribute or an event.

Attributes can be customized with the `#[attribute]` annotation with:

* `name` to set the HTML attribute name.
  By default, it's the kebab-case of the parameter name.
* `option` to mark the attribute optional.
  `true` by default if the type is `Option<...>`.
* `initial` to set the default value when the HTML attribute is missing
  By default use the `std::defaultDefault` implementation of the type.
* `parse` to provide the conversion between the HTML attribute value (a string) to the type value.
  By default use the `std::str::FromStr` implementation, and fall to the default value if it fails.


Events are parameters with the Dioxus `EventHandler<...>` type.
You can customize the event with these attributes:

* `name` to set the HTML event name.
  By default use the parameter name without the `on` prefix (if any)
* `no_bubble` to forbid the custom event to bubble
* `no_cancel` to remove the ability to cancel the custom event


This example uses all annotations:

```rust, ignore
use dioxus::prelude::*;
use dioxus_web_component::web_component;

#[web_component]
fn MyWebComponent(
    #[attribute(name= "attr1", option = false, initial = String::new(), parse = |value| Some(value.to_string()))]
    attr1: String,
    #[attribute(name = "attr-option", option = true, initial = None, parse = |value| Some(value.to_string()))]
    attr_option: Option<String>,
    #[event(name = "event", no_bubble = false, no_cancel = false)] event: EventHandler<i64>,
) -> Element {
    todo!()
}

```

See [dioxus-web-component-macro] documentation for more details.

## Usage without macro

<details>
<summary>The usage without macro is discouraged</summary>

You can provide your manual implementation of [`DioxusWebComponent`] and call
[`register_dioxus_web_component`] to register your web component.

The key point is to use a `Shared` element in the dioxus context.


For example, the greeting example could be written with

```rust, ignore,
use dioxus::prelude::*;
use dioxus_web_component::{
    register_dioxus_web_component, DioxusWebComponent, InjectedStyle, Shared,
};
use wasm_bindgen::prelude::*;

/// Install (register) the web component
///
/// # Errors
///
/// Registering the web-component may fail
#[wasm_bindgen(start)]
pub fn register() -> Result<(), JsValue> {
    register_greetings();

    Ok(())
}

// #[web_component(tag = "plop-greeting", style = InjectedStyle::css(include_str!("./style.css"))  )]
#[component]
fn Greetings(name: String) -> Element {
    rsx! { p { "Hello {name}!" } }
}

#[derive(Clone, Copy)]
struct GreetingWebComponent {
    name: Signal<String>,
}

impl DioxusWebComponent for GreetingWebComponent {
    fn set_attribute(&mut self, attribute: &str, value: Option<String>) {
        if attribute == "name" {
            let value = value.unwrap_or_default();

            self.name.set(value);
        }
    }
}

fn greetings_builder() -> Element {
    let name = use_signal(String::default);
    let context = use_context::<Shared>();
    let wc = GreetingWebComponent { name };
    context.init_component(wc);

    rsx! {
        Greetings { name }
    }
}

fn register_greetings() {
    let attributes = vec!["name".to_string()];
    let style = InjectedStyle::css(include_str!("./style.css"));

    register_dioxus_web_component("plop-greeting", attributes, style, greetings_builder);
}
```

The counter example looks like this:

```rust, ignore
use dioxus::prelude::*;
use dioxus_web_component::{
    custom_event_handler, register_dioxus_web_component, CustomEventOptions,
    DioxusWebComponent, InjectedStyle, Shared,
};
use wasm_bindgen::prelude::*;

/// Install (register) the web component
///
/// # Errors
///
/// Registering the web-component may fail
#[wasm_bindgen(start)]
pub fn register() -> Result<(), JsValue> {
    // The register counter is generated by the `#[web_component(...)]` macro
    register_counter();
    Ok(())
}

/// The Dioxus component
// #[web_component(tag = "plop-counter", style = InjectedStyle::stylesheet("./style.css"))]
#[component]
fn Counter(on_count: EventHandler<i32>) -> Element {
    let mut counter = use_signal(|| 0);

    rsx! {
        button {
            onclick: move |_| {
                counter += 1;
                on_count(counter());
            },
            "+"
        }
        output { "{counter}" }
    }
}

#[derive(Clone, Copy)]
struct CounterWebComponent {
    on_count: EventHandler<i32>,
}

impl DioxusWebComponent for CounterWebComponent {
    fn set_attribute(&mut self, _attribute: &str, _value: Option<String>) {
        // nop
    }
}

fn counter_builder() -> Element {
    let mut context = use_context::<Shared>();
    let on_count = custom_event_handler(
        context.event_target(),
        "count",
        CustomEventOptions::default(),
    );
    let wc = CounterWebComponent { on_count };
    context.init_component(wc);

    rsx! {
        Counter{ on_count }
    }
}

fn register_counter() {
    let attributes = vec![];
    let style = InjectedStyle::stylesheet("./style.css");

    register_dioxus_web_component("plop-counter", attributes, style, counter_builder);
}

```

</details>

## Limitations

* web component properties not (yet) supported
* only extends `HTMLElement`
* only work as a replacement of Dioxus `#[component]` annotation (does not work with handmade `Props`)
* no validation of the custom element name


## Contributions

Contributions are welcome ❤️.


[Dioxus]: https://dioxuslabs.com/
[web component]: https://developer.mozilla.org/en-US/docs/Web/API/Web_components
[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen
[npm]: https://www.npmjs.com/
[wasm-pack]: https://github.com/rustwasm/wasm-pack
[Rust WebAssembly book]: https://rustwasm.github.io/docs/book/
[dioxus-web-component-macro]: https://github.com/ilaborie/dioxus-web-component/blob/main/dioxus-web-component-macro/README.md
