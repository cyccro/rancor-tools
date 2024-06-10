import Renderer from "./Renderer.js";
import { changeProp } from "./Events.js";
import RuntimeVar from "./RuntimeVar.js";
import { redef } from "./helpful.js";
import { Option } from "../types.js";

const configObject = {
  get(t: any, p: string) {
    return t.element[p] ?? t[p];
  },
  set(t: any, p: string, v: any) {
    if (Reflect.has(t, p)) t[p] = v;
    else t.element[p] = v, changeProp.emit({
      element: t.element,
      property: p,
      value: v
    });
    return true;
  }
};
interface ElementProps {
  [key: string]: string;
}
export default class Element extends Renderer {
  linkedVars = new Map;
  el_children = new Set as Set<Element>;
  parent: Option<Element> = null;
  constructor(element: HTMLElement, properties: ElementProps) {
    super(element);
    const result = new Proxy(this, configObject);
    properties.textContent ??= properties.txt;
    properties.className = properties.className ?? properties.cls ?? properties.class ?? properties.clss;
    for (const key in properties) if (element[key] != null) result.element[key] = properties[key];
    return result;
  }
  link(variable: RuntimeVar, prop: string) {
    if (this.linkedVars.has(prop)) return new Error("Cannot link more than 1 var to the same property");
    variable.onChange(v => this[prop] = v.value ?? '');
    return this;
  }
  setLinks(links: ElementProps) {
    for (const link in links) {
      if (this.linkedVars.has(link)) return new Error(`Cannot link more than 1 var to the same property, trying to link ${link}`);
      this.linkedVars.set(link, links[link].onChange(v => this[link] = v.value ?? ''));
    }
    return this;
  }
  addElement(el: Element | HTMLElement | string | Text): Element {
    if (typeof el == 'string') return this.addElement(document.createTextNode(el));
    const og = el;
    this.el_children.add(el as Element);
    if (el instanceof Element) while (el instanceof Element) {
      el.parent = this;
      if (el.element instanceof Element) el.element.parent = el;
      el = el.element;
    };
    try {
      this.element.appendChild(el as HTMLElement);
    } catch (e) {
      this.el_children.delete(og as Element);
      throw new Error(`Could not append child. Child must be an Element or Node, instead got ${og == void (0) ? 'null' : og.constructor.name}`);
    }
    return this;
  }
  modifyProp(prop: string | ElementProps, value: any) {
    if (typeof prop == 'object') for (const key in prop) this[prop] = prop[key];
    else this[prop] = value;
    return this;
  }
  include(...element: Element[]) {
    if (element.length == 1 && element[0] instanceof Array) for (const el of element[0]) this.addElement(el);
    else for (const el of element) this.addElement(el);
    return this;
  }
  onClick(fn: (element: Element, parent: Element['parent'], ev: MouseEvent) => void) {
    this.element.addEventListener("click", (ev) => fn(this, this.parent, ev));
    return this;
  }
  exclude(...element: Element[]) {
    for (let el of element) {
      this.el_children.delete(el);
      el.parent = null;
      while (el instanceof Element) (el.element instanceof Element && (el.element.parent = el), el = el.element);
      this.element.removeChild(el);
    }
    return this;
  }
  has(element: Element) {
    return this.el_children.has(element);
  }
  //is child of
  ichof(element: Element) {
    return element.has(this);
  }
  render(target: HTMLElement) {
    super.render(target);
    return this;
  }
  *[Symbol.iterator](): Generator<HTMLElement, void, Element> {
    yield this.element;
    for (const child of this.el_children) yield* child;
  }
  clearChildren(filter: (el: Element) => boolean, errfn: (e: unknown) => any) {
    if (!this.el_children) return true;
    try {
      for (const element of this.el_children)
        if (filter?.(element) ?? true) this.exclude(element);
      return true;
    } catch (e) {
      const result = errfn?.(e) ?? console.log(e);
      return result ?? false;
    }
  }
  static Button = class ButtonElement extends this {
    constructor(properties: ElementProps) {
      super(document.createElement('button'), properties);
    }
  };
  static Div = class DivElement extends this {
    constructor(properties: ElementProps) {
      super(document.createElement("div"), properties);
    }
  };
  static H1 = class H1 extends this {
    constructor(properties: ElementProps) {
      super(document.createElement("h1"), properties);
    }
  };
  static H2 = class H2 extends this {
    constructor(properties: ElementProps) {
      super(document.createElement("h2"), properties);
    }
  };
  static Input = class Input extends this {
    private connection: Option<RuntimeVar> = null;
    constructor(properties: ElementProps) {
      super(document.createElement("input"), redef(properties, {
        password: {
          type: "password"
        },
        info: {
          canDelete: true
        }
      }));
    }
    onInput(fn: any) {
      this.element.addEventListener('input', fn);
      return this;
    }
    onChange(fn: any) {
      this.element.addEventListener('change', fn);
      return this;
    }
    connect(v: RuntimeVar) {
      if (this.connection) throw new Error("Cannot set 2 connections");
      if (v instanceof RuntimeVar) {
        this.connection = v;
        this.onInput((e: any) => {
          v.set(e.target.value);
        });
        return this;
      }
      throw new Error("The given variable is not a RuntimeVar");

    }
  };
  static Anchor = class Anchor extends this {
    declare element: HTMLParagraphElement & { href: string };
    constructor(properties: ElementProps) {
      super(document.createElement("p"), properties)
      this.element.href = properties.href;
      this.element.addEventListener('click', () => {
        const last = window.location.href.at(-1);
        if (this.element.href == '/') {
          window.location.href = window.location.origin;
          return;
        }
        if (last == '/') {
          if (this.element.href[0] == '/')
            window.location.href += this.element.href.substring(1);
          else throw new Error(`Possible invalid href ${this.element.href}`);
        } else {
          if (this.element.href[0] == '/') {
            window.location.href += this.element.href;
          } else throw new Error("Invalid hrefs");
        }
      })
    }
  }
  static A = class A extends this {
    constructor(properties: ElementProps) {
      super(document.createElement('a'), properties);
    }
  }
  static Link = class Link extends this.A {
    constructor(properties: ElementProps) {
      properties.className ? properties.className += " link" : properties.className = "link";
      super(properties);
    }
  }
  static P = class P extends this {
    constructor(properties: ElementProps) {
      super(document.createElement("p"), properties);
    }
  }
  static Image = class Image extends this {
    static href(link: string) {
      return new this({
        src: link
      })
    }
    constructor(properties: ElementProps) {
      super(document.createElement("img"), properties);
    }
  }
  static Form = class Form extends this {
    declare element: HTMLFormElement;
    constructor(properties: ElementProps) {
      super(document.createElement("form"), properties);
    }
    action_href(href: string) {
      this.setAttribute('action', href);
      return this;
    }
    method(method = 'GET') {
      this.setAttribute('method', method);
      return this;
    }
    submit(fn = void (0)) {
      fn && (this.element.onsubmit = fn);
      this.element.submit();
      return this;
    }
  }
}
