import CustomizedEvent from "./Events.js";
function defaultValue(type: any): string | boolean | number {
  switch (type) {
    case 'string': return '';
    case 'number': return 0;
    case 'boolean': return false;
  }
}

export default class RuntimeVar {
  static Change = new CustomizedEvent("change");
  static Update = new CustomizedEvent("update");
  private changeHandlers: ((variable: this) => void)[] = [];
  private updateHandlers: ((variable: this) => void)[] = [];
  value: string | number | boolean;
  caster: NumberConstructor | StringConstructor | BooleanConstructor;
  constructor(public type: 'string' | 'boolean' | 'number') {
    if ((this.value = defaultValue(type)) == void (0)) throw new Error(`Not recognized primitive type '${type}'`);
    this.caster = type == 'string' ? String : type == 'number' ? Number : Boolean;
  }
  valueOf() {
    return this.value;
  }
  toString() {
    return this.value.toString();
  }
  set(v: string | boolean | number) {
    if (typeof v !== this.type) {
      v = this.caster(v);
      if (!v) throw new Error(`Cannot set value of type ${typeof v} in a runtime var of type ${this.type}`);
    }
    RuntimeVar.Change.emit({
      variable: this,
      oldValue: this.value,
      newValue: v
    });
    this.value = v;
    this.changeHandlers.forEach(fn => fn(this));
  }
  update(fn: (value: string | number | boolean) => string | number | boolean) {
    const result = fn(this.value);
    if (typeof result !== this.type)
      return new Error(`Cannot set value of type ${typeof result} in a runtime var of type ${this.type}`);
    RuntimeVar.Update.emit({
      variable: this,
      oldValue: this.value,
      newValue: result
    });
    this.value = result;
    this.updateHandlers.forEach(fn => fn(this));
  }
  onChange(fn: (variable: this) => void) {
    this.changeHandlers.push(fn);
    return this;
  }
  onUpdate(fn: (variable: this) => void) {
    this.updateHandlers.push(fn);
    return this;
  }
  clear() {
    switch (this.type) {
      case 'string': this.value = ''; break;
      case 'number': this.value = 0; break;
      case 'boolean': this.value = false;
    }
    return this;
  }
}
