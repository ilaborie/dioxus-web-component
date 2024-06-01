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

The parameters of the component could be:

* an __attribute__ if you want to pass the parameter as an HTML attribute,
* a __property__ if you only want to read/write the parameter as a property of the Javascript `HTMLElement`,
* or an __event__ if the parameter is a Dioxus `EventHandler`.

💡TIP: You can be an attribute AND a property if you use the two annotations.

#### Attributes

Attributes can be customized with the `#[attribute]` annotation with:

* `name` to set the HTML attribute name.
  By default, it's the kebab-case of the parameter name.
* `option` to mark the attribute optional.
  `true` by default if the type is `Option<...>`.
* `initial` to set the default value when the HTML attribute is missing
  By default use the `std::default::Default` implementation of the type.
* `parse` to provide the conversion between the HTML attribute value (a string) to the type value.
  By default use the `std::str::FromStr` implementation, and fall to the default value if it fails.


#### Property

To declare a property, you need to use the `#[property]` annotation.

We use [wasm-bindgen] to convert the Rust side value to a Javascript value.

⚠️ IMPORTANT: The getter returns a Javascript `Promise`.

You can customize the property with these attributes:

* `name` to set the Javascript name of the property.
  By default, it's the camelCase of the parameter name.
* `readonly` to only generate the custom getter
* `initial` to set the default value when the HTML attribute is missing
  By default use the `std::defaultDefault` implementation of the type.
* `try_from_js` to provide the conversion from a `JsValue` to the parameter type.
  By default use the `std::convert::TryInto` implementation.
  The error case is ignored (does not set the value)
* `try_into_js` to provide the conversion from the parameter type to a `JsValue`.
  By default use the `std::convert::TryInto` implementation.
  Return `undefined` in case of error


#### Events

Events are parameters with the Dioxus `EventHandler<...>` type.
You can customize the event with these attributes:

* `name` to set the HTML event name.
  By default use the parameter name without the `on` prefix (if any)
* `no_bubble` to forbid the custom event from bubbling
* `no_cancel` to remove the ability to cancel the custom event


This example uses all annotations:

```rust, ignore
use dioxus::prelude::*;
use dioxus_web_component::{web_component, InjectedStyle};

#[web_component(tag = "my-component", style = InjectedStyle::css(include_str!("./style.css")))]
fn MyWebComponent(
    #[attribute(name = "attr1", option = false, initial = String::new(), parse = |value| Some(value.to_string()))]
    attr1: String,
    #[attribute(name = "attr-option", option = true, initial = None, parse = |value| Some(value.to_string()))]
    attr_option: Option<String>,
    // Readonly property
    #[property(readonly)]
    prop: Option<String>,
    // Property with custom conversion
    #[property(
        initial = MyProp(true),
        try_into_js = |prop| {
            let js_value = if prop.0 {
                JsValue::TRUE
            } else {
                JsValue::FALSE
            };
            Ok::<_, Infallible>(js_value)
        },
        try_from_js= |value| Ok::<_, Infallible>(MyProp(value.is_truthy())),
    )]
    prop2: MyProp,
    #[event(name = "event", no_bubble = false, no_cancel = false)] event: EventHandler<i64>,
) -> Element {
    todo!()
}

#[derive(Clone, PartialEq)]
struct MyProp(bool);
```

See [dioxus-web-component-macro] documentation for more details.

## Usage without macro

Currently, the idea is to avoid breaking changes when you use the macros,
but you should expect to have some in the API.

<details>
<summary>The usage without macro is discouraged</summary>

You can provide your manual implementation of [`DioxusWebComponent`] and call
[`register_dioxus_web_component`] to register your web component.

The key point is to use a `Shared` element in the dioxus context.


For example, the greeting example could be written with

```rust, ignore,
use dioxus::prelude::*;
use dioxus_web_component::{
    register_dioxus_web_component, DioxusWebComponent, InjectedStyle, Message, Property, Shared,
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

#[component]
fn Greetings(name: String) -> Element {
    rsx! { p { "Hello {name}!" } }
}


fn register_greetings() {
    let properties = vec![Property::new("name", false)];
    let style = InjectedStyle::css(include_str!("./style.css"));
    register_dioxus_web_component(
        "plop-greeting",
        vec!["name".to_string()],
        properties,
        style,
        greetings_builder,
    );
}

#[derive(Clone, Copy)]
struct GreetingsWebComponent {
    name: Signal<String>,
}

impl DioxusWebComponent for GreetingsWebComponent {
    fn set_attribute(&mut self, attribute: &str, value: Option<String>) {
        match attribute {
            "name" => {
                let new_value = value.and_then(|attr| attr.parse().ok()).unwrap_or_default();
                self.name.set(new_value);
            }
            _ => {
                // nop
            }
        }
    }

    fn set_property(&mut self, property: &str, value: JsValue) {
        match property {
            // we allow to set the name as a property
            "name" => {
                if let Ok(new_value) = Ok(value).and_then(|value| value.try_into()) {
                    self.name.set(new_value);
                }
            }
            _ => {
                // nop
            }
        }
    }

    fn get_property(&mut self, property: &str) -> JsValue {
        match property {
            // we allow to get the name as a property
            "name" => Ok(self.name.read().clone())
                .and_then(|value| value.try_into())
                .unwrap_or(::wasm_bindgen::JsValue::NULL),
            _ => JsValue::undefined(),
        }
    }
}

fn greetings_builder() -> Element {
    let mut wc = use_context::<Shared>();
    let name = use_signal(String::new);
    let mut greetings = GreetingsWebComponent { name };
    let corountine = use_coroutine::<Message, _, _>(move |mut rx| async move {
        use dioxus_web_component::StreamExt;
        while let Some(msg) = rx.next().await {
            greetings.handle_message(msg);
        }
    });

    use_effect(move || {
        wc.set_tx(corountine.tx());
    });

    rsx! {
        Greetings {
            name
        }
    }
}

```

The counter example looks like this:

```rust, ignore
use dioxus::prelude::*;
use dioxus_web_component::{
    custom_event_handler, register_dioxus_web_component, CustomEventOptions, DioxusWebComponent,
};
use dioxus_web_component::{InjectedStyle, Message, Property, Shared};
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
#[component]
fn Counter(label: String, on_count: EventHandler<i32>) -> Element {
    let mut counter = use_signal(|| 0);

    rsx! {
        span { "{label}" }
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

fn register_counter() {
    let properties = vec![Property::new("label", false)];
    let style = InjectedStyle::stylesheet("./style.css");
    register_dioxus_web_component("plop-counter", vec![], properties, style, counter_builder);
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct CounterWebComponent {
    label: Signal<String>,
    on_count: EventHandler<i32>,
}

impl DioxusWebComponent for CounterWebComponent {
    #[allow(clippy::single_match_else)]
    fn set_property(&mut self, property: &str, value: JsValue) {
        match property {
            "label" => {
                let new_value = String::(value).unwrap_throw();
                self.label.set(new_value);
            }
            _ => {
                // nop
            }
        }
    }

    #[allow(clippy::single_match_else)]
    fn get_property(&mut self, property: &str) -> JsValue {
        match property {
            "label" => {
                let value = self.label.read().clone();
                value.into()
            }
            _ => JsValue::undefined(),
        }
    }
}

fn counter_builder() -> Element {
    let mut wc = use_context::<Shared>();
    let label = use_signal(String::new);
    let on_count = custom_event_handler(wc.event_target(), "count", CustomEventOptions::default());

    let mut counter = CounterWebComponent { label, on_count };
    let corountine = use_coroutine::<Message, _, _>(move |mut rx| async move {
        use dioxus_web_component::StreamExt;
        while let Some(msg) = rx.next().await {
            counter.handle_message(msg);
        }
    });

    use_effect(move || {
        wc.set_tx(corountine.tx());
    });

    rsx! {
        Counter {
            label,
            on_count
        }
    }
}
```

</details>

## Limitations

* only extends `HTMLElement`
* only work as a replacement of Dioxus `#[component]` annotation (does not work with handmade `Props`)
* cannot add a method callable from Javascript in the web component. (Workaround: use property)


## Contributions

Contributions are welcome ❤️.


[Dioxus]: https://dioxuslabs.com/
[web component]: https://developer.mozilla.org/en-US/docs/Web/API/Web_components
[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen
[npm]: https://www.npmjs.com/
[wasm-pack]: https://github.com/rustwasm/wasm-pack
[Rust WebAssembly book]: https://rustwasm.github.io/docs/book/
[dioxus-web-component-macro]: https://github.com/ilaborie/dioxus-web-component/blob/main/dioxus-web-component-macro/README.md
