interface NodeSaver {
  target: HTMLElement;
  nodes: HTMLElement[];
}
export default class Renderer {
  savedNodes = {} as NodeSaver;
  constructor(public element: HTMLElement) {
  }
  render(target = document.body) {
    if (this.savedNodes) {
      Array.from(target.childNodes).map(v => v.cloneNode(true));
      (this.savedNodes.target == target) && this.savedNodes.nodes.forEach(node => document.body.appendChild(node));
    } else target.appendChild(this.element);
    return this;
  }
  is_in(target = document.body) {
    const nodes = target.childNodes;
    const set = new Set(nodes);
    const el = set.has(this.element);
    return el ? set : void (0);
  }
  unrender(target = document.body, save = false) {
    let nodes = this.is_in(target);
    if (nodes) {
      if (save) this.savedNodes.set({
        target,
        nodes
      });
      target.removeChild(this.element);
    }
    return this;
  }
}
