import { getRouteData, getRoutes } from "@/api/files";
import Image from "next/image";
export default function Home() {
  const routes = getRoutes().map(route => {
    const data = getRouteData(route);
    const img = data.img ? <img src={"data:image/png;base64," + data.img} /> : void (0);
    return <div key={data.routeName}>
      <h1>{data.routePath}</h1>
      {img}
    </div>
  });
  return (
    <div>
      {routes}
    </div>
  );
}
