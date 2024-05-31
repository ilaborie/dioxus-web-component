export function register_web_component(custom_tag, rust_component) {
  customElements.define(
    custom_tag,
    class extends HTMLElement {
      static get observedAttributes() {
        return rust_component.attributes;
      }

      constructor() {
        super();
        this.attachShadow({ mode: "open" });
        const instance = rust_component.newInstance(this.shadowRoot);
        for (let prop of rust_component.properties) {
          const {name, readonly} = prop;
          if (readonly) {
            Object.defineProperty(this, name, {
              get() {
                return instance.getProperty(name);
              }
            });
          } else {
            Object.defineProperty(this, name, {
              get() {
                return instance.getProperty(name);
              },
              set(value) {
                instance.setProperty(name, value);
              }
            });
          }
        }
        this.instance = instance;
      }

      attributeChangedCallback(name, oldValue, newValue) {
        this.instance.attributeChanged(name, oldValue, newValue);
      }

      connectedCallback() {
        this.instance.connect(this);
      }

      disconnectedCallback() {
        this.instance.disconnect();
      }
    }
  );
}
