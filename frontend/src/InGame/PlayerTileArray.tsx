import { Player } from "../state/playersSlice";
import PlayerTile from "./PlayerTile";

type PlayerTileArryProps = {
    players: Array<Player>,
    setSelectedPlayerID: (id: string) => void,
    selectedPlayerID: string | undefined
}

export default function PlayerTileArray(props: PlayerTileArryProps) {

    return (<div className="justify-left flex flex-wrap place-content-center">
        {props.players.map((player) => {
            return (
                <div key={player.id} onClick={() => props.setSelectedPlayerID(player.id)}>
                    <PlayerTile  {...player} isSelected={player.id === props.selectedPlayerID} />
                </div>
            )
        })}
    </div>)
}
