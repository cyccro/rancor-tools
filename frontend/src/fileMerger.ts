import { FileData } from "./fileManager";

export enum FileType {
  ADDON,
  BEHAVIOR,
  RESOURCE
}

async function write(file: Promise<FileSystemWritableFileStream>, content: string) {
  return file.then((f) => {
    f.write(content);
    return f.close();
  });
}

export default class FileMerger {
  constructor(public handler: FileSystemDirectoryHandle) { };
  async createFile(path: string,) {
    const splitPath = path.split("/");
    if (splitPath.at(-1) == '/') splitPath.splice(splitPath.length - 1, 1);
    let subfolder = this.handler;
    for (let i = 0, j = splitPath.length - 1; i < j; i++) subfolder = await subfolder.getDirectoryHandle(splitPath[i], { create: true });
    return subfolder.getFileHandle(splitPath.at(-1), { create: true });
  }
  createFiles(files: FileData[]) {
    files.forEach(async file => write((await this.createFile(`${file.name}/${file.parent}`)).createWritable(), file.file_data));
  }
}
