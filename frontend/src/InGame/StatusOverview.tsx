import PlayerTile from "./PlayerTile";
import { useAppSelector } from "../hooks";

type StatusOverviewProps = {};

function StatusOverview(_props: StatusOverviewProps) {
    const { players } = useAppSelector((state) => state.players);
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
