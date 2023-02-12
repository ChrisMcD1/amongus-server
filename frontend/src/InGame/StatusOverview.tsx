import { useContext } from "react";
import { v4 as uuidV4 } from "uuid";
import { PlayersContext } from "../App";
import PlayerTile from "./PlayerTile";

export type Player = {
  color: string;
  name: string;
  alive: boolean;
  id: ReturnType<typeof uuidV4>;
};

type StatusOverviewProps = {};

function StatusOverview(_props: StatusOverviewProps) {
  const { players } = useContext(PlayersContext)!;
  return (
    <div className="flex h-screen flex-col place-content-center justify-center">
      <h1 className="mx-auto mt-0 mb-10">Game Overview</h1>
      <div className="justify-left flex flex-wrap place-content-center">
        {players.map((player) => (
          <PlayerTile key={player.id} {...player} />
        ))}
      </div>
    </div>
  );
}

export default StatusOverview;
