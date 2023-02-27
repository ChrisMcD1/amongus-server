import PlayerTile from "./PlayerTile";
import ImposterAbilities from "./ImposterAbilities";
import { useAppSelector } from "../hooks";
import { selectCurrentPlayer, selectOtherPlayers } from "../state/playersSlice";

type StatusOverviewProps = {};

function StatusOverview(_props: StatusOverviewProps) {
    const currentPlayer = useAppSelector(selectCurrentPlayer);
    const otherPlayers = useAppSelector(selectOtherPlayers);
    return (
        <div className="flex h-screen flex-col place-content-center justify-center p-5">
            <div>
                <div className="justify-center flex flex-wrap place-content-center">
                    {otherPlayers.map((player) => (
                        <PlayerTile key={player.id} {...player} />
                    ))}

                </div>
            </div>
            {currentPlayer?.role === "imposter" && <ImposterAbilities />}
        </div>
    );
}

export default StatusOverview;
