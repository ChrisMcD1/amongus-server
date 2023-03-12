import { createKillPlayerMessage } from "../Messages/toServer";

export type ImposterAbilitiesProps = {
    selectedPlayerID?: string,
    ws: WebSocket | undefined
}

export default function ImposterAbilities(props: ImposterAbilitiesProps) {
}
