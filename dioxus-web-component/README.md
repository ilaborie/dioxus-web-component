# Dioxus Web Component

This create provide a bridge to expose a [Dioxus] component as a [web-component].

This crate support web component attributes, and custom events.
You can also add CSS style to your web component.

Take a look at the examples to see the usage in a full project:
<https://github.com/ilaborie/dioxus-web-component/tree/main/examples>


If you are new to WebAssembly with Rust, take a look at the [Rust WebAssembly book] first.

## Usage with macro

Ideally, you only need to replace the Dioxus `#[component]` by `#[web_component]`.
Then you should register the web component with [wasm-bindgen].
And to finish, you can create the [npm] package with [wasm-pack].


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
  By default it's the kebab case version of the function name.
  ⚠️ There are some restriction to custom element names, see [MDN - Valid custom element names](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define#valid_custom_element_names)
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
  By default use the `std::str::FromStr` implementation, and fallback to the default value if it's fail.


Event are parameters with the Dioxus `EventHandler<...>` type.
You can customize the event with these attributes:

* `name` to set the HTML event name.
  By default use the parameter name without the `on` prefix (if any)
* `no_bubble` to forbib the custom event to bubble
* `no_cancel` to remove the ability to cancel the custom event


This example use all annotations:

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


For example, the greeting example could be written with

```rust, ignore,
use std::borrow::Cow::Borrowed;

use dioxus::prelude::*;
use dioxus_web_component::{register_dioxus_web_component, Context, DioxusWebComponent, Message};
use dioxus_web_component::{
    register_dioxus_web_component, Context, DioxusWebComponent, InjectedStyle, Message,
};
use wasm_bindgen::prelude::*;

/// Install (register) the web component
///
/// # Errors
///
/// Registering the web-component may fail
#[wasm_bindgen(start)]
pub fn register() -> Result<(), JsValue> {
    register_dioxus_web_component::<GreetingsWebComponent>("plop-greeting");
    Ok(())
}

/// The Dioxus component
#[component]
fn Greetings(name: String) -> Element {
    rsx! { p { "Hello {name}!" } }
}

struct GreetingsWebComponent;

impl DioxusWebComponent for GreetingsWebComponent {
    fn style() -> InjectedStyle {
        let css = include_str!("./style.css");
        InjectedStyle::Css(Borrowed(css))
    }

    fn attributes() -> &'static [&'static str] {
        &["name"]
    }

    fn element() -> Element {
        let mut name_signal = use_signal(String::new);
        let Context { rx, .. } = use_context();
        let _change_handler = use_coroutine::<(), _, _>(|_| async move {
            while let Ok(Message::AttributeChanged { new_value, .. }) = rx.recv().await {
                let value = new_value.unwrap_or_else(|| "World".to_owned());
                name_signal.set(value);
            }
        });
        rsx! { Greetings { name: name_signal } }
    }
}
```

And the counter example looks like:

```rust, ignore
use std::borrow::Cow;

use dioxus::prelude::*;
use dioxus_web_component::{
    custom_event_handler, register_dioxus_web_component, Context, CustomEventOptions,
    DioxusWebComponent, InjectedStyle,
};
use wasm_bindgen::prelude::*;

/// Install (register) the web component
///
/// # Errors
///
/// Registering the web-component may fail
#[wasm_bindgen(start)]
pub fn register() -> Result<(), JsValue> {
    register_dioxus_web_component::<CounterWebComponent>("plop-counter");

    Ok(())
}

/// The Dioxus component
#[component]
fn Counter(on_count: EventHandler<i32>) -> Element {
    let mut counter = use_signal(|| 0);

    rsx! {
        button {
            onclick: move |_| {
                counter += 1;
                on_count(*counter.read());
            },
            "+"
        }
        output { "{counter}" }
    }
}

struct CounterWebComponent;

impl DioxusWebComponent for CounterWebComponent {
    fn style() -> InjectedStyle {
        let url = Cow::Borrowed("./style.css");
        InjectedStyle::Stylesheet(url)
    }

    fn attributes() -> &'static [&'static str] {
        &["name"]
    }

    fn element() -> Element {
        let Context { event_target, .. } = use_context();
        let on_count = custom_event_handler(event_target, "count", CustomEventOptions::default());

        rsx! { Counter { on_count } }
    }
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
[web-component]: https://developer.mozilla.org/en-US/docs/Web/API/Web_components
[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen
[npm]: https://www.npmjs.com/
[wasm-pack]: https://github.com/rustwasm/wasm-pack
[Rust WebAssembly book]: https://rustwasm.github.io/docs/book/
[dioxus-web-component-macro]: https://github.com/ilaborie/dioxus-web-component/blob/main/dioxus-web-component-macro/README.md
