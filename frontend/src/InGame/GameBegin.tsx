import React, { useState } from "react";
type BeginProps = { username: string };

export default function GameBegin(props: BeginProps) {
  return (
    <div className="background flex h-screen items-center bg-black">
      <div className="flex h-fit">
        <img src="https://media.tenor.com/qCqLC7df-eIAAAAC/among-us-shhhhhhh.gif" />
      </div>
    </div>
  );
}
