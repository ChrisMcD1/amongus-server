import { Player } from "../state/playersSlice"
import Color from "color";
import { ReactComponent as AmongusMan } from "../JoinGame/Whitetest.svg";

type PlayerTileProps = Player & {
    isSelected: boolean,
    showBorder: boolean,
    className: string
}

export default function PlayerTile(props: PlayerTileProps) {
    var selectedBorderColor = (props.isSelected ? 'border-red-500' : 'border-white');
    var borderClasses = `rounded-2xl border-2 border-solid ${selectedBorderColor}`;
    return (
        <div className={`m-1 h-full flex flex-col items-center p-2 ${props.showBorder ? borderClasses : ""} ${props.className}`}>
            <div style={props.alive ? { color: 'white' } : { color: 'red' }} className="font-amongus-log text-2xl text-white">{props.username}</div>
            <div className="flex-shrink">
                <AmongusMan
                    style={{
                        ["--base-color" as any]: props.color, ["--shadow-color" as any]: Color(props.color).darken(0.3)
                    }}
                    className="player mx-auto max-h-full max-w-none items-center"
                />
            </div>

        </div>
    );
}
