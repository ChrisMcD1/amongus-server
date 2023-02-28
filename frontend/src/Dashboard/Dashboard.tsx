import start from "../../public/Pictures/start.png";

export default function Dashboard() {

    function startGame() {
        fetch(`http://${import.meta.env.VITE_BACKEND_SERVER}/start-game`, { method: "POST" });
    };

    function resetGame() {
        fetch(`http://${import.meta.env.VITE_BACKEND_SERVER}/reset-game`, { method: "POST" });
    }

    return (
        <div>
            <button
                className="mx-auto bg-transparent"
            >
                <img src={start} onClick={startGame} />
            </button>
            <button className="text-lg m-4" onClick={resetGame}>
                Reset Game
            </button>
        </div>
    )
}
