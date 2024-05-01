use dioxus::prelude::EventHandler;
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::{CustomEvent, EventTarget};

/// HTML custom event options
///
/// See [MDN - custom event](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent)
///
/// Note that by default `can_bubble` & `cancelable` are `true`
#[derive(Clone, Copy)]
pub struct CustomEventOptions {
    /// Is the event bubble up through the DOM tree
    ///
    /// See [MDN - bubbles](https://developer.mozilla.org/en-US/docs/Web/API/Event/bubbles)
    pub can_bubble: bool,

    /// Is the event is cancelable
    ///
    /// See [MDN - cancelable](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelable)
    pub cancelable: bool,
}

impl Default for CustomEventOptions {
    fn default() -> Self {
        Self {
            can_bubble: true,
            cancelable: true,
        }
    }
}

/// Create a Dioxus event handler that send an HTML custom event
pub fn custom_event_handler<T>(
    target: impl AsRef<EventTarget> + 'static,
    event_type: &'static str,
    options: CustomEventOptions,
) -> EventHandler<T>
where
    T: Into<JsValue> + 'static,
{
    EventHandler::new(move |value: T| {
        let CustomEventOptions {
            can_bubble,
            cancelable,
        } = options;
        let event = CustomEvent::new(event_type).unwrap_throw();
        let detail = value.into();
        event.init_custom_event_with_can_bubble_and_cancelable_and_detail(
            event_type, can_bubble, cancelable, &detail,
        );
        let target = target.as_ref();
        target.dispatch_event(&event).unwrap_throw();
    })
}
