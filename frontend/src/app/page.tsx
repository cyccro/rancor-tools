"use client";
import { AddonType } from "@/api/Addons";
import { uuid } from "@/api/others";
import DropDown from "@/components/DropDown";
import FolderInput from "@/components/FolderInput";

//A request with more than N files for some reason stop the requests, for security, 50 will be the max
const maxFiles = 50;

async function finishDownload(uuid: string) {
  const url = "http://localhost:8080/finish_merge/"+uuid;
  try{
    const response = await fetch(url, {
      method:'POST'
    });
    const blob = await response.blob();
    console.log(blob);
    const anchor = document.createElement('a');
    anchor.href = URL.createObjectURL(blob);
    anchor.download = "merge.zip";
    document.body.appendChild(anchor);
    anchor.click();
    document.body.removeChild(anchor);
    URL.revokeObjectURL(anchor.href);
  }catch(e){
    console.log(e, 'error');
  }
}

async function download(r1: File[], type: AddonType) {
  const id = uuid();
  const len = r1.length;
  if(len == 0) return console.log("No files were given");
  let i = len;
  console.time("label");
  const fn = async () => {
    if (i <= 0) {
      await finishDownload(id);
      clearInterval(interval);
      console.timeEnd("label");
      return;
    }
    const form = new FormData();
    const this_lim = (i - maxFiles) * Number(i > maxFiles);
    for (; i > this_lim; i--)
      if (i < 0) return;
      else form.append('r1', r1[i]);

    try {
      const res = await fetch("http://localhost:8080/merge/" + id, {
        method: 'POST',
        body: form
      });
      console.log(await res.text());
      setTimeout(fn,125);
    } catch (e) {
      console.warn(e, "Given error during sending things.")
      console.timeEnd("label");
      clearInterval(interval);
    }
  }
  let interval = setTimeout(fn, 125);

}
export default function Home() {
  const r1: File[] = [];
  let addonType = 0 as AddonType;
  return (
    <div className="h-screen text-white flex mt-20 items-center flex-col">
      <h1 className="text-2xl font-semibold">Merging addons with a bit of magic!</h1>
      <div className="flex flex-col mt-5">
        <div className="flex flex-row gap-x-5">
          <FolderInput name="inp1" onInput={(files) => r1.push(...files)} />
          <FolderInput name="inp2" onInput={(files) => r1.push(...files)} />
        </div>
        <DropDown name="addon_type" dropname="Select Addon Type" contents={["Behavior pack", "Resource pack", "Complete Addon"]} onSelect={(ev) => {
          switch (ev.target.value) {
            case 'Behavior pack': addonType = AddonType.Beh; break;
            case 'Resource pack': addonType = AddonType.Res; break;
            case 'Complete Addon': addonType = AddonType.Addon; break;
          }
        }} />
        <button onClick={() => download(r1, addonType)}>Download merge</button>
      </div>
    </div>
  );
}
