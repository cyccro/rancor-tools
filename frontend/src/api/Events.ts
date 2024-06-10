export default class CustomizedEvent {
  #fns = new Set;
  constructor(type) {
    this.type = type;
  }
  addFn(...fns) {
    for (const fn of fns) this.#fns.add(fn);
  }
  emit(o) {
    for (const fn of this.#fns) fn(o);
  }
}
export const changeProp = new CustomizedEvent("changeProp");
