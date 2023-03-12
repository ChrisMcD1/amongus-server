import { useAppSelector } from "../hooks";
import { selectCurrentPlayer, selectOtherPlayers } from "../state/playersSlice";
import { useState } from "react";
import { createEmergencyMeetingMessage, createKillPlayerMessage, createReportBodyMessage } from "../Messages/toServer";
import PlayerTileArray from "./PlayerTileArray";

type StatusOverviewProps = { ws: WebSocket | undefined };

function StatusOverview(props: StatusOverviewProps) {
    const currentPlayer = useAppSelector(selectCurrentPlayer);
    const otherPlayers = useAppSelector(selectOtherPlayers);
    const [selectedPlayerID, setSelectedPlayerID] = useState<string | undefined>(undefined);
    function callEmergencyMeeting() {
        props.ws?.send(createEmergencyMeetingMessage());
    }
    function killCrewmate() {
        if (props.ws == null) {
            throw new Error("Cannot call kill crewmate without a websocket");
        }
        if (selectedPlayerID == null) {
            console.warn("Its a bit silly to kill without a target");
            return;
        }
        props.ws.send(createKillPlayerMessage(selectedPlayerID));
    }
    function reportBody() {
        if (props.ws == null) {
            throw new Error("Cannot call report crewmate without a websocket");
        }
        if (selectedPlayerID == null) {
            console.warn("Its a bit silly to report without a target");
            return;
        }
        props.ws.send(createReportBodyMessage(selectedPlayerID));
    }
    return (
        <div className="flex h-screen w-screen flex-col place-content-center justify-center p-5">
            <button onClick={callEmergencyMeeting}>
                Call Emergency Meeting!
            </button>
            <PlayerTileArray players={otherPlayers} setSelectedPlayerID={setSelectedPlayerID} selectedPlayerID={selectedPlayerID} />

            {currentPlayer?.role === "imposter" &&
                <div>I'm an imposter :)</div>
            }
            <div className="grid grid-cols-2 gap-2">
                <button onClick={killCrewmate}>Kill Target </button>
                <button onClick={reportBody}>Report Body</button>
            </div>
        </div >
    );
}

export default StatusOverview;
