export { };

export function createReportBodyMessage(corpseID: string) {
    return createMessage("ReportBody", {
        corpse: corpseID
    })
}

export function createKillPlayerMessage(targetID: string) {
    return createMessage("KillPlayer", {
        target: targetID
    })
}

export function createVoteMessage(targetID: string) {
    return createMessage("Vote", {
        target: targetID
    })
}

export function createColorMessage(color: string) {
    return createMessage("ChooseColor", {
        color,
    });
}

function createMessage(type: string, content: any) {
    return JSON.stringify({
        type,
        content
    })
}

