export default function define<T>(target: T, property: string, value: PropertyKey): T {
  if (!value) return define(window, target as any as string, property);
  Object.defineProperty(target, property, {
    value,
    writable: false
  });
  return target;
};
interface RedefObj {
  info?: {
    canDel?: boolean;
    canDelete?: boolean;
    defvoid?: boolean;
  }
  [key: string]: any;
}
export function redef(object: Record<string, any>, props: RedefObj): typeof object {
  const canDel = props.info?.canDel || props.info?.canDelete;
  if (props.info?.defvoid) for (const key in props) object[key] = props;
  else for (const key in props) {
    if (typeof props[key] == 'object') {
      const prop = props[key];
      if (object[key]) {
        for (const otkey in prop) object[otkey] = prop[otkey];
        if (canDel) delete props[key];
      }
      continue;
    }
    if (object[key] != void (0) && props[key] != void (0)) {
      object[key] = props[key];
      continue;
    }

  }
  return object;
}
const protObj = Reflect.getPrototypeOf({});
export function isObjLit(obj: any) {
  try {
    return Reflect.getPrototypeOf(obj) == protObj;
  } catch {
    return false;
  }
}
export function uuid4() {
  return "10000000-1000-4000-8000-100000000000".replace(/[018]/g, c =>
    (+c ^ crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> +c / 4).toString(16));
}

export function goto(url: string, replace = false) {
  if (replace) {
    window.location.href = url;
    return;
  }
  const urlsplit = url.split("/");
  if (urlsplit.length > 2)
    for (const split of urlsplit) goto(split);

  if (url[0] == '/') {
    if (window.location.href.at(-1) == '/') window.location.href += url.substring(1);
    else window.location.href += url;
  }
  else if (url == '..') {
    const splitl = window.location.href.split('/').at(-1).length;
    window.location.href = window.location.href.substring(0, window.location.href.length - splitl);
  }
  else window.location.href = url;
}

export async function wait(ms) {
  return new Promise(r => setTimeout(() => r(), ms));
}
export async function req(route = '', data) {
  if (!route && !data) return fetch(serverUrl);
  if (!data) return fetch(serverUrl, route);
  if (!route) return fetch(serverUrl, data);

  return fetch(serverUrl + "/" + route, data);
}
