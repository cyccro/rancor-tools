import Image from "next/image";
import RedirectButton from "@/components/RedirectButton";
import { getRoutes } from "@/api/routes";
export default function Home() {
  return (
    <div className="flex flex-col items-center">
      <Image></Image>
      <div className="flex flex-col teste justify-center items-center min-h-screen">
        <div className="flex gap-x-5">
          {getRoutes().map(route =>
            <RedirectButton key={route.name} link={route.link}>
              {route.img ? (
                <div className="flex flex-col items-center justify-center">
                  <Image src={route.img} alt={route.name + ' logo'} width="150" height="800" />
                  <h1 className="text-xl">{route.content}</h1>
                </div>) : route.name}
            </RedirectButton>)}
        </div>
      </div>
    </div>
  );
}
