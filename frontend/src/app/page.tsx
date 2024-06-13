"use client";
import { AddonType } from "@/api/Addons";
import DropDown from "@/components/DropDown";
import FolderInput from "@/components/FolderInput";

async function download(r1: File[], r2: File[], type: AddonType) {
  const form = new FormData();
  r1.forEach(file => form.append('r1', file));
  r2.forEach(file => form.append('r2', file));
  const data = {
    method: 'POST',
    body: form
  };
  try {
    const res = await fetch("http://localhost:8080/merge", data);
    if (res.ok) {
      //console.log(await res.text(), 'response');
      const blob = await res.blob();
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'merged_addons.zip';
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      window.URL.revokeObjectURL(url);
    } else {
      throw 'non knowed'
    }
  } catch (e) {
    console.error(e, 'error kkk');
  }
}
export default function Home() {
  let r1: File[] = [];
  let r2: File[] = [];
  let addonType = 0;
  return (
    <div className="h-screen flex mt-20 items-center flex-col">
      <h1 className="text-2xl font-semibold">Merging addons with a bit of magic!</h1>
      <div className="flex flex-col mt-5">
        <div className="flex flex-row gap-x-5">
          <FolderInput name="inp1" onInput={(files) => r1.push(...files)} />
          <FolderInput name="inp2" onInput={(files) => r2.push(...files)} />
        </div>
        <DropDown name="addon_type" dropname="Select Addon Type" contents={["Behavior pack", "Resource pack", "Complete Addon"]} onSelect={(ev) => {
          addonType = AddonType[ev.target.value as any] as any as number;
          console.log(addonType);
        }} />
        <button onClick={() => download(r1, r2, addonType)}>Download merge</button>
      </div>
    </div>
  );
}
