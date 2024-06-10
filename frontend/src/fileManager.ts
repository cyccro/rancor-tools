import type { Option, fs, future } from "./types";

function getFileType(filename: string) {
  return filename.split('.').at(-1);
}

export class FileData {
  name: string;
  is_file: boolean;
  file_data: string;
  file_type: string;
  parent: string;
  constructor(file: File, filecontent: string, parent: string) {
    this.name = file.name;
    this.is_file = true;
    this.file_data = filecontent;
    this.file_type = getFileType(file.name);
    this.parent = parent;
  }
}

export class Folder {
  static async new(): future<Option<Folder>> {
    try {
      if (!window.showDirectoryPicker) throw 0;
      const picker = await window.showDirectoryPicker();
      return new this(picker, '');
    } catch (e) {
      if (e == 0)
        alert("Probably the browser you are using does not support directory imports, please go to one which allows it!");
    }
  }
  public name: string;
  constructor(public handler: FileSystemDirectoryHandle, parentname: string) {
    this.name = (parentname + '/' + handler.name);
    if (this.name[0] == '/') this.name = this.name.substring(1);
  }
  get folders() {
    return this.getAllSubfolders();
  }
  get files() {
    return this.getAllFiles();
  }
  /**
   * Gets the subfolder with the given name inside this one. If create = true, it will create the folder if it does not find it and returns it
  */
  async getSubfolder(dirname: string, create: boolean = false): future<Option<Folder>> {
    if (!create) try {
      return new Folder(await this.handler.getDirectoryHandle(dirname, { create }), this.name);
    } catch {
      console.warn(`Didn't find a (sub)directory named as ${dirname} in the given directory`);
    }
    else return new Folder(await this.handler.getDirectoryHandle(dirname, { create }), this.name);
  }
  /**
   * Gets the file with the given name inside this folder
   */
  async getFile(filename: string): future<Option<fs['File']>> {
    try {
      return await this.handler.getFileHandle(filename);
    } catch {
      console.warn(`Didn't find a file named as ${filename} in the given directory`);
    }
  }
  /**
  * Gets all the files inside this folder
  */
  async getAllFiles(fn: (file: FileData) => void): future<FileData[]> {
    const files: FileData[] = [];
    let current: File;
    let current_data: FileData;
    for await (const handler of this.handler.values())
      if (handler.kind == 'file') {
        current = await handler.getFile() as File;
        current_data = new FileData(current, await current.text(), this.name);
        fn(current_data);
        files.push(current_data);
      }
    return files;
  }
  /**
   * Gets all subfolders inside this one
   */
  async getAllSubfolders() {
    const subfolders: Folder[] = [];
    let folder: Folder;
    for await (const [name, handler] of this.handler) {
      if (handler.kind == 'directory') {
        folder = (await this.getSubfolder(name))!;
        subfolders.push(folder);
      }
    }
    return subfolders;
  }
  /**
  * Gets all files inside this folder and the files inside the subfolders and so on.
  */
  async getEveryFiles(target: FileData[] | void, fn: (data: FileData) => void): future<FileData[]> {
    if (!target) {
      const files = await this.getAllFiles(fn);
      for (const folder of await this.folders) folder.getEveryFiles(files, fn);
      return files;
    }
    else {
      target.push(...await this.getAllFiles(fn));
      for (const folder of await this.folders) folder.getEveryFiles(target, fn);
    }
    return target;
  }
}
