import { useContext } from "react";
import { useNavigate } from "react-router-dom";
import { PlayersContext } from "../App";
import { configureWebsocket } from "../websocket";
type HomeProps = {
  username: string;
  setUsername: (name: string) => void;
  setWs: (ws: WebSocket) => void;
};

export default function Home(props: HomeProps) {
  const handleChange = (e: any) => {
    props.setUsername(e.target.value);
  };
  const navigate = useNavigate();
  const playerContext = useContext(PlayersContext)!;
  const joinGame = async (_e: any) => {
    let ws = new WebSocket(
      `ws://localhost:9090/join-game?username=${props.username}`
    );
    ws = configureWebsocket(ws, playerContext);
    props.setWs(ws);
    navigate("/lobby");
  };
  return (
    <div className="flex h-screen w-screen flex-col justify-center bg-space-stars bg-cover bg-fixed bg-center bg-no-repeat">
      <button
        className="center mx-auto border-white bg-transparent py-5 px-10 font-amongus-text text-5xl"
        onClick={joinGame}
      >
        Join Game
      </button>
      <div className="mx-auto flex p-10">
        <label
          className="user-label mx-5 font-amongus-title text-4xl"
          htmlFor="name"
        >
          Username:
        </label>
        <input
          onChange={handleChange}
          className="user-input rounded-lg border border-white bg-transparent font-amongus-log"
          type="text"
          defaultValue={props.username}
          required
          minLength={1}
          maxLength={10}
          size={12}
        ></input>
      </div>
    </div>
  );
}
