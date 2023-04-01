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
    const [showRole, setShowRole] = useState(false);
    return (
        <div className="bg-gray-700 flex h-screen w-screen flex-col place-content-center justify-center p-5">
            <button className="mt-5 text-white bg-gray-600" onClick={() => setShowRole(!showRole)}>
                {showRole ? "Hide Role" : "Show Role"}
            </button>
            {showRole &&
                <div className="text-center text-white">I am a {currentPlayer?.role}!</div>
            }
            <h1 className="align-self-start text-center m-4 text-white">{currentPlayer?.username} {currentPlayer?.alive === false ? "(I'm a ghost)" : ""}</h1>
            <button className="my-2 text-white bg-gray-600" onClick={callEmergencyMeeting}>
                Call Emergency Meeting!
            </button>
            <div className="flex-shrink min-h-0">
                <PlayerTileArray players={otherPlayers} setSelectedPlayerID={setSelectedPlayerID} selectedPlayerID={selectedPlayerID} />
            </div>

            <div className="grid grid-cols-2 gap-2">
                <button className="bg-gray-600 text-white" onClick={killCrewmate}>Kill Target </button>
                <button className="bg-gray-600 text-white" onClick={reportBody}>Report Body</button>
            </div>
        </div >
    );
}

export default StatusOverview;
