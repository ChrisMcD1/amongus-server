export default function Dashboard() {
    function resetGame() {
        fetch(`http://${import.meta.env.VITE_BACKEND_SERVER}/reset-game`, { method: "POST" });
    }
    return (
        <div className="mx-auto flex w-screen flex-col justify-center align-middle">
            <h1 className="m-10 text-center">Dashboard</h1>
            <button className="mx-auto px-10" onClick={resetGame}>
                Reset Game
            </button>
        </div>
    );
}
