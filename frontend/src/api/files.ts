import fs from "fs";
import path from "path";

type Option<T> = T | void;

interface RouteData {
	routePath: string,
	img: Option<string>
	routeName: string
}

export const APP_PATH = "./src/app";

export function getDirNamesInApp() {
	const foldernames: string[] = [];
	const files = fs.readdirSync(APP_PATH);
	let currPath: string;

	for (const name of files) {
		if (!path.extname(currPath = APP_PATH + '/' + name)) //if not, is a dir
			foldernames.push(currPath);
	}
	return foldernames;
}
export function isRouteInMain(route: string) {
	return fs.readdirSync(route).includes("mainRoute.ignore");
}

export function getRoutes() {
	return getDirNamesInApp().filter(isRouteInMain);
}

export function getRouteData(route: string): RouteData {
	const minifiedRoute = route.split("/").slice(3).join("/");
	const imgpath = route + "/routeImg.png";
	return {
		routePath: minifiedRoute,
		routeName: route,
		img: fs.existsSync(imgpath) ? fs.readFileSync(imgpath).toString("base64") : void (0)
	}
}
