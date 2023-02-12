import { Player } from "./StatusOverview";

export default function PlayerTile(props: Player) {
    return (
        <div className="m-1 border-white border-2 border-solid rounded-xl p-2 flex flex-col items-center" >
            <div style={{ backgroundColor: props.color }} className="rounded-full h-20 w-20 mx-auto" ></div>
            <div>
                {props.name}
            </div>
            <div>
                {props.alive ? "I'm alive" : "I'm dead"}
            </div>
        </div>
    )
}
