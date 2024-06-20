export interface Route {
	img?: string,
	name: string,
	link: string,
	content?: string
};

export function getRoutes(): Route[] {
	const routes = [{ //i could but donot want to do this automatic
		link: 'merger',
		name: 'Merger',
		content: 'Merger',
		img: './folder-svgrepo-com.svg'
	}];
	routes.forEach(route => route.link = './' + route.link);

	return routes;
}
