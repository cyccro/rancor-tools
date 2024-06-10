import Element from "./api/Elements";

export class Render extends HTMLElement {
  connectedCallback() {
    const attr = this.getAttribute("r-id") || "r1";
    this.appendChild(new Element.Div({
      class: 'reader'
    }).include(
      new Element.Div({
        class: attr
      }).include(
        new Element.Button({
          class: 'btn-reader',
          id: attr,
        }).include(
          new Element.Image({
            srcset: "./svg/folder.svg",
            class: "folder",
            alt: "Folder image"
          })
        )
      ),
      new Element.Div({
        class: "",
      }).include(
        new Element.H2({
          class: "priority",
          textContent: "Has priority?"
        }),
        new Element.Input({
          id: attr + "-priority",
          type: "checkbox"
        })
      )
    ).element);
  }
}
customElements.define("rct-reader", Render);
