"use client";
import { Dispatch, SetStateAction, useState } from "react";
import Image from "next/image";
export default function Home() {
  const [r1, setR1] = useState([])
  const [r2, setR2] = useState([]);

  function handleR1(ev: any) {
    setR1(Array.from(ev.target.files))
  }
  function handleR2(ev: any) {
    setR2(Array.from(ev.target.files))
  }
  async function download() {
    const form = new FormData();
    (r1 as File[]).forEach(file => form.append('r1', file));
    (r2 as File[]).forEach(file => form.append('r2', file));
    const data = {
      method: 'POST',
      body: form
    };
    try {
      const res = await fetch("http://localhost:8080/helloworld", data);
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
  return (
    <div>
      <div className="flex flex-col gap-y-3">
        <input type="file" multiple onChange={handleR1} webkitdirectory="true" />
        <input type="file" multiple onChange={handleR2} webkitdirectory="true" />
        <button onClick={download}>Download merge</button>
      </div>
    </div>
  );
}
