import Link from "next/link";
import React from "react";
export default function RedirectButton({ children, link, className }: { children?: React.ReactNode, link: string, className?: string }) {
	return (
		<Link href={link} className={className}>
			<div className="cursor-pointer border-2 px-5 py-3 my-3 rounded border-fuchsia-700 bg-purple-600 duration-300 hover:border-purple-800 hover:bg-violet-900">
				{children}
			</div>
		</Link>
	);
}
