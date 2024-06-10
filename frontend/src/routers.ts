class Router {
    private routes: { [key: string]: string };

    constructor() {
        this.routes = {};
        window.addEventListener('hashchange', this.route.bind(this));
    }

    addRoute(route: string, url: string) {
        this.routes[route] = url;
    }

    async route() {
        const hash = window.location.hash.substring(1);
        const url = this.routes[hash];
        if (url) {
            const response = await fetch(url);
            const content = await response.text();
            document.getElementById('content').innerHTML = content;
        }
    }
}

const router = new Router();

router.addRoute('home', './src/pages/home.html');
router.addRoute('addons', './src/pages/addons.html');
router.addRoute('merge', './src/pages/merge.html');
router.addRoute('creatorApis', './src/pages/creatorApis.html');

// Default route
window.addEventListener('load', () => {
    if (!window.location.hash) {
        window.location.hash = '#home';
    }
    router.route();
});
