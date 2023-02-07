import { v4 as uuidV4 } from "uuid";
import PlayerTile from "./PlayerTile"

type StatusOverviewProps = {
    ws: WebSocket | undefined;
}

function StatusOverview(props: StatusOverviewProps) {
    let samplePlayerID = uuidV4();
    return <div className="flex flex-col h-screen place-content-center justify-center">
        <h1 className="mt-0 mx-auto mb-10">Game Overview</h1>
        <p>{props?.ws?.readyState}</p>
        <div className="flex justify-left flex-wrap place-content-center">
            <PlayerTile color={"#0F0"} id={samplePlayerID} name={"Chris"} alive={true} />
            <PlayerTile color={"#0FF"} id={samplePlayerID} name={"Kate"} alive={true} />
            <PlayerTile color={"#FF0"} id={samplePlayerID} name={"Steven"} alive={false} />
            <PlayerTile color={"#F0F"} id={samplePlayerID} name={"Jordan"} alive={false} />
            <PlayerTile color={"#0F0"} id={samplePlayerID} name={"Jenny"} alive={true} />
        </div>
    </div>
}

export default StatusOverview
