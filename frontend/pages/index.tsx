import React from 'react';
import Image from 'next/image';
import Link from 'next/link';

const Home = () => {
  return (
    <header>
      <Image
        className="logo"
        src="/images/logo.png"
        alt="logo"
        width={500}
        height={500}
      />
      <div className="cardHome">
        <Link href="/merge">
          <button className="btnstyle btnhome">
            <Image src="/svg/folder.svg" alt="" width={24} height={24} />
            Addon Merger
          </button>
        </Link>
        <Link href="/addons">
          <button className="btnstyle btnhome">addons</button>
        </Link>
        <Link href="/creatorApis">
          <button className="btnstyle btnhome">creator APIs</button>
        </Link>
      </div>
    </header>
  );
};

export default Home;
