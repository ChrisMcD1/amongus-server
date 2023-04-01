import { Player } from "../state/playersSlice";
import PlayerTile from "./PlayerTile";

type PlayerTileArryProps = {
    players: Array<Player>,
    setSelectedPlayerID: (id: string) => void,
    selectedPlayerID: string | undefined
}

export default function PlayerTileArray(props: PlayerTileArryProps) {

    return (
        <div className="justify-left flex flex-row flex-wrap h-full">
            {props.players.map((player) => {
                return (
                    <div className="max-h-full max-w-full w-1/3 p-1" key={player.id} onClick={() => props.setSelectedPlayerID(player.id)}>
                        <PlayerTile  {...player} isSelected={player.id === props.selectedPlayerID} showBorder={true} className="" />
                    </div>
                )
            })}
        </div>
    )
}
