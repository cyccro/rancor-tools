export type Option<T> = T | void | null;
export type fs = {
  Dir: FileSystemDirectoryHandle,
  File: FileSystemFileHandle
}
export type future<T> = Promise<T>;

