import React, { useState } from "react";
import { useAppSelector } from "../hooks";
import { ReactComponent as Whitetest } from "../JoinGame/Whitetest.svg";

export default function RoleAssignment() {
  return (
    <div className="background flex h-screen w-screen items-center bg-black bg-[url('../../public/Pictures/impostertemplate.jpg')] bg-contain bg-center bg-no-repeat">
      <div className="absolute top-[50%] flex w-screen flex-col items-center justify-items-center">
        <div className="z-20">
          <Whitetest className="player center  h-24 md:h-32" />
        </div>
        <h3 className="inset-y-2/3 z-20  font-amongus-log text-white md:text-lg">
          {useAppSelector((state) => state.user.user)}
        </h3>
      </div>
    </div>
  );
}
