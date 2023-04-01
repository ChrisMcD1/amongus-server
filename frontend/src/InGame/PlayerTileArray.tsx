import { Player } from "../state/playersSlice";
import PlayerTile from "./PlayerTile";

type PlayerTileArryProps = {
    players: Array<Player>,
    setSelectedPlayerID: (id: string) => void,
    selectedPlayerID: string | undefined
}

export default function PlayerTileArray(props: PlayerTileArryProps) {

    return (<div className="justify-left grid grid-cols-3">
        {props.players.map((player) => {
            return (
                <div className="h-full" key={player.id} onClick={() => props.setSelectedPlayerID(player.id)}>
                    <PlayerTile  {...player} isSelected={player.id === props.selectedPlayerID} showBorder={true} className="" />
                </div>
            )
        })}
    </div>)
}
