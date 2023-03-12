import { Player } from "../state/playersSlice"

export default function PlayerTile(props: Player) {
    return (
        <div className="m-1 flex flex-col items-center rounded-xl border-2 border-solid border-white p-2">
            <div
                style={{ backgroundColor: props.color }}
                className="mx-auto h-20 w-20 rounded-full"
            ></div>

            <div style={props.alive ? { color: 'white' } : { color: 'red' }}>{props.username}</div>
        </div>
    );
}
