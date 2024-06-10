import img from "next/image"

export default function Home() {
  return (
    <main>
      <header>
      <img
          className="logo"
          src="frontend/public/images/logo.png"
          alt="logo"
          width={500}
          height={500}
        />
          <nav className="navHome">
            <button className="btnstyle btnhome">fww</button>
            <button className="btnstyle btnhome">fe</button>
            <button className="btnstyle btnhome">ef</button>
          </nav>
      </header>
    </main>
  );
}