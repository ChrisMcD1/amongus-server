import { createKillPlayerMessage } from "../Messages/toServer";

export type ImposterAbilitiesProps = {
    selectedPlayerID?: string,
    ws: WebSocket | undefined
}

export default function ImposterAbilities(props: ImposterAbilitiesProps) {
    function killCrewmate() {
        if (props.ws == null) {
            throw new Error("Cannot call kill crewmate without a websocket");
        }
        if (props.selectedPlayerID == null) {
            console.warn("Its a bit silly to kill without a target");
            return;
        }
        props.ws.send(createKillPlayerMessage(props.selectedPlayerID));
    }
    return (<div>
        <div>I'm an imposter :)</div>
        <p>Got my sights set on {props.selectedPlayerID}</p>
        <button onClick={killCrewmate}>Kill Them </button>
    </div>);
}
