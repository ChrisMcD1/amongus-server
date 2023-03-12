import { Player } from "../state/playersSlice"

type PlayerTileProps = Player & {
    isSelected: boolean
}

export default function PlayerTile(props: PlayerTileProps) {
    return (
        <div className={`m-1 flex flex-col items-center rounded-xl border-2 border-solid ${props.isSelected ? 'border-red-500' : 'border-white'} p-2`}>
            <div
                style={{ backgroundColor: props.color }}
                className="mx-auto h-20 w-20 rounded-full"
            ></div>

            <div style={props.alive ? { color: 'white' } : { color: 'red' }}>{props.username}</div>
        </div>
    );
}
