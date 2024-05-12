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
        this.instance = rust_component.newInstance(this.shadowRoot);
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
