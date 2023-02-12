

export default function Dashboard() {
    function resetGame() {
        fetch("http://localhost:9090/reset-game", { method: 'POST' });
    }
    return (
        <div className="flex flex-col mx-auto justify-center align-middle w-screen">
            <h1 className="m-10 text-center">Dashboard</h1>
            <button className="mx-auto px-10" onClick={resetGame}>Reset Game</button>
        </div>
    )
}
