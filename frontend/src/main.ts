import { FileData, Folder } from './fileManager';
import './style.css';
import "./customTags";

function modifyCheck(self: HTMLInputElement, input: HTMLInputElement) {
  if (self.checked) input.checked = false;
}
function modifyTitle(title: HTMLElement, files: any[], index: number) {
  const interval = setInterval(() => (title.innerText == `${files.length} files were readen by reader ${index}`) ? clearInterval(interval) : title.innerText = `${files.length} files were readen by reader ${index}`, 100);
}
main: {
  {
    const first_folder_priority = document.getElementById("r1-priority") as HTMLInputElement;
    const second_folder_priority = document.getElementById("r2-priority") as HTMLInputElement;
    first_folder_priority.addEventListener("click", () => modifyCheck(first_folder_priority, second_folder_priority));
    second_folder_priority.addEventListener("click", () => modifyCheck(second_folder_priority, first_folder_priority));
  };
  {
    const r1 = document.getElementById("r1") as HTMLButtonElement;
    const r2 = document.getElementById("r2") as HTMLButtonElement;
    const title = document.getElementsByClassName("title")[0] as HTMLElement;
    const r1files = [] as FileData[];
    const r2files = [] as FileData[];
    r1.addEventListener("click", async () => {
      if (r1files[0])
        r1files.length *= Number(!confirm("You have already selected a folder in this reader, you sure want to empty it to get other files?"));
      //boolean -> false = 0, true = 1
      //if confirm == true, r1files.length = r1files.length * (false || 0);
      //else r1files.length = r1files.length * (true || 1) == r1files.length
      //js normal não precisa dar casting(Number(bool)), ele aceita operacoes numericas com booleans por padrão, já que booleans são u8
      const folder = await Folder.new();
      if (!folder) return;
      await folder.getEveryFiles(r1files, (file) => title.innerText = "Reading file: " + file.name).then(() => modifyTitle(title, r1files, 1));
    });
    r2.addEventListener("click", async () => {
      if (r2files[0]) r2files.length *= Number(!confirm("You have already selected a folder in this reader, you sure want to empty it to get other files?"));
      const folder = await Folder.new();
      if (!folder) return;
      await folder.getEveryFiles(r2files, (file) => title.innerText = "Reading file: " + file.name).then(() => modifyTitle(title, r2files, 2));
    });
  }
}
