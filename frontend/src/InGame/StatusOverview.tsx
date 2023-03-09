import PlayerTile from "./PlayerTile";
import ImposterAbilities from "./ImposterAbilities";
import { useAppSelector } from "../hooks";
import { selectCurrentPlayer, selectOtherPlayers } from "../state/playersSlice";
import { useState } from "react";
import { createEmergencyMeetingMessage } from "../Messages/toServer";

type StatusOverviewProps = { ws: WebSocket | undefined };

function StatusOverview(props: StatusOverviewProps) {
    const currentPlayer = useAppSelector(selectCurrentPlayer);
    const otherPlayers = useAppSelector(selectOtherPlayers);
    const [selectedPlayerID, setSelectedPlayerID] = useState<string | undefined>(undefined);
    function callEmergencyMeeting() {
        props.ws?.send(createEmergencyMeetingMessage());
    }
    return (
        <div className="flex h-screen w-screen flex-col place-content-center justify-center p-5">
            <button onClick={callEmergencyMeeting}>
                Call Emergency Meeting!
            </button>
            <div>
                <div className="justify-left flex flex-wrap place-content-center">
                    {otherPlayers.map((player) => {
                        return (
                            <div key={player.id} onClick={() => setSelectedPlayerID(player.id)}>
                                <PlayerTile  {...player} />
                            </div>
                        )
                    })}

                </div>
            </div>
            {currentPlayer?.role === "imposter" && <ImposterAbilities selectedPlayerID={selectedPlayerID} ws={props.ws} />}
        </div >
    );
}

export default StatusOverview;
