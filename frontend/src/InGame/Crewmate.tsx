import React from "react";
type CrewmateProps = { username: string };

export default function Crewmate(props: CrewmateProps) {
  return (
    <div className="background flex">
      <div className="flex">
        <label className="user-label" htmlFor="name">
          Username:
        </label>
      </div>
    </div>
  );
}
