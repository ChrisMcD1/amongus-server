export default function EmergencyButton() {
    function callEmergencyMeeting() {
        console.log("Emergency Time!");
    }
    return (
        <div className="w-screen h-screen flex items-center justify-center">
            <button className="h-min" onClick={callEmergencyMeeting}>
                Call Emergency Meeting!
            </button>
        </div>
    )
}
