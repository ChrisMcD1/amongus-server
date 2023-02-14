import { Player } from "./StatusOverview";

export default function PlayerTile(props: Player) {
  console.log(props);
  return (
    <div className="m-1 flex flex-col items-center rounded-xl border-2 border-solid border-white p-2">
      <div
        style={{ backgroundColor: props.color }}
        className="mx-auto h-20 w-20 rounded-full"
      ></div>
      <div>{props.name}</div>
      <div>{props.alive ? "I'm alive" : "I'm dead"}</div>
    </div>
  );
}
