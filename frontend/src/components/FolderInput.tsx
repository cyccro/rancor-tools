"use client";
import { useState } from "react";

export default function FolderInput({ name, onInput }: { name: string, onInput: (files: File[]) => void }) {
	const [fileLen, setlen] = useState(0);
	const [files, setfiles] = useState([] as File[]);
	const handleChange = (ev: InputEvent) => {
		const _files = (ev.target as HTMLInputElement).files!;
		const len = _files.length;
		setlen(len);
		onInput(Array.from(_files));
	}
	return (
		<div>
			<h2 className="font-medium text-xl">{fileLen} files selected</h2>
			<div className="cursor-pointer border-2 px-5 py-3 my-3 rounded border-fuchsia-700 bg-purple-600 duration-300 hover:border-purple-800 hover:bg-violet-900">
				<label htmlFor={name} className="cursor-pointer">
					<img src="./folder-svgrepo-com.svg" alt="input folder" className="w-32" />
				</label>
			</div>
			<input type="file" id={name} onChange={handleChange} webkitdirectory="true" className="hidden" />
		</div>
	);
}
