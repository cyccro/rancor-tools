import React from 'react';
import Link from 'next/link';
import Image from 'next/image';

const Merge = () => {
  return (
    <>
      <header>
        <nav className="navHome">
          <Link href="/">
            <button className="btnstyle">
              <Image src="/images/home-pixelizada.png" alt="home" width={24} height={24} />
            </button>
          </Link>
        </nav>
      </header>
      <div id="app">
        <div>
          <h2 className="title">Merge your addons into once with a bit of magic!</h2>
        </div>
        <div className="readers">
          <div className="reader" id="r1"></div>
          <div className="reader" id="r2"></div>
        </div>
        <button className="btnstyle" id="confirmation">Confirm ></button>
      </div>
    </>
  );
};

export default Merge;
