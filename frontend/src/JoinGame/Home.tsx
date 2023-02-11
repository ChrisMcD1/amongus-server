import { useNavigate } from "react-router-dom";
type HomeProps = {
    username: string,
    sendName: (name: string) => void;
    setWs: (ws: WebSocket) => void;
}

export default function Home(props: HomeProps) {
    const handleChange = (e: any) => {
        props.sendName(e.target.value);
    }
    const navigate = useNavigate();
    const joinGame = async (_e: any) => {
        let ws = new WebSocket(`ws://localhost:9090/join-game?username=${props.username}`);
        ws.onopen = () => {
            console.log("Websocket has opened!");
        }
        ws.onmessage = (msg: any) => {
            console.log(JSON.parse(msg.data));
        }
        props.setWs(ws);
        navigate("/lobby");
    }
    return (
        <div className="bg-space-stars justify-center bg-center flex flex-col h-screen w-screen bg-cover bg-no-repeat bg-fixed">
            <button className='center mx-auto text-2xl py-5 px-10' onClick={joinGame}>
                Join Game
            </button>
            <div className='flex mx-auto p-10'>
                <label className='user-label mx-5 text-2xl' htmlFor="name">Username:</label>
                <input onChange={handleChange} className='user-input'
                    type="text" defaultValue={props.username}
                    required minLength={1} maxLength={10} size={12}></input>
            </div>
        </div>
    );
}

