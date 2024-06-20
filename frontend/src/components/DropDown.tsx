import { ChangeEventHandler } from "react";

export default function DropDown({ name, dropname, contents, onSelect }: { name: string, dropname: string, contents: string[], onSelect: ChangeEventHandler<HTMLSelectElement> }) {
	return (
		<div>
			<label htmlFor={name}>{dropname}</label>
			<select name={name} id={name} onChange={onSelect}>
				{contents.map(content =>
					<option value={content} key={content}>{content}</option>
				)}
			</select>
		</div>
	)
}
