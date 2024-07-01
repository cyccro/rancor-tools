"use client";
import { AddonType } from "@/api/Addons";
import { uuid } from "@/api/others";
import DropDown from "@/components/DropDown";
import FolderInput from "@/components/FolderInput";

async function download(r1: File[], type: AddonType) {
  const id = uuid();

  const form = new FormData();
  for (const file of r1) form.append("r1", file);
  try {
    const res = await fetch("http://localhost:8080/merge/" + id, {
      method: 'POST',
      body: form
    });
    const blob = await res.blob();
    const anchor = document.createElement('a');
    anchor.href = URL.createObjectURL(blob);
    anchor.download = "merge.zip";
    document.body.appendChild(anchor);
    anchor.click();
    document.body.removeChild(anchor);
    URL.revokeObjectURL(anchor.href);
  } catch (e) {
    console.warn(e, "Given error during sending things.");
  }
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
