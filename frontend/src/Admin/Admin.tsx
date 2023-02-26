import start from "../../public/Pictures/start.png";

export default function Admin() {

    function startGame() {
        fetch("http://localhost:9090/start-game", { method: "POST" });
    };

    function resetGame() {
        fetch("http://localhost:9090/reset-game", { method: "POST" });
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
