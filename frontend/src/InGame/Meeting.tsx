import { useState } from "react";
import { useAppSelector } from "../hooks";
import { createVoteMessage } from "../Messages/toServer";
import { selectMeetingInitiator } from "../state/meetingSlice";
import { selectCurrentPlayer, selectOtherPlayers } from "../state/playersSlice";
import PlayerTileArray from "./PlayerTileArray";

type MeetingProps = { ws: WebSocket | undefined };

export default function Meeting(props: MeetingProps) {
    const currentPlayer = useAppSelector(selectCurrentPlayer);
    const otherPlayers = useAppSelector(selectOtherPlayers);
    const initatingPlayer = useAppSelector(selectMeetingInitiator)
    const purpose = useAppSelector((state) => state.meeting.purpose);
    const [selectedPlayerID, setSelectedPlayerID] = useState<string | undefined>(undefined);
    function voteForSelectedPlayer() {
        if (selectedPlayerID == null) {
            return;
        }
        props.ws?.send(createVoteMessage(selectedPlayerID));
    }
    function skipVote() {
        props.ws?.send(createVoteMessage(null));
    }
    return (
        <div className="flex h-screen w-screen flex-col place-content-center justify-center p-5 bg-gray-700">
            <h1 className="text-4xl mx-auto mb-5 text-white">
                {
                    purpose === "Emergency" ? "Emergency Meeting" : "Body Reported"
                }
            </h1>
            <h2 className="text-xl mx-auto mb-5 text-white">Initiator: {initatingPlayer?.username}</h2>
            <h3 className="text-lg text-center text-white">{currentPlayer?.username}</h3>
            <PlayerTileArray players={otherPlayers} setSelectedPlayerID={setSelectedPlayerID} selectedPlayerID={selectedPlayerID} />
            <div className="grid grid-cols-2 my-5">
                <button className="mx-2 text-white bg-gray-600" style={selectedPlayerID == null ? { pointerEvents: 'none' } : {}} onClick={voteForSelectedPlayer} >Vote</button>
                <button className="mx-2 text-white bg-gray-600" onClick={skipVote} >Skip</button>
            </div>
        </div >
    );
}
