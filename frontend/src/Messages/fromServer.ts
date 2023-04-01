import { z } from "zod";

export const PreZodMessage = z.object({
    type: z.enum([
        "gameState",
        "resetGame",
        "chatMessage",
        "assignedID",
        "playerStatus",
        "playerRole",
        "playerDied",
        "successfulKill",
        "invalidAction",
        "votingResults",
        "gameOver"]),
    content: z.unknown()
});

export const ChatMessage = z.object({
    contents: z.string()
})

export const PlayerConnectionStatus = z.enum(["new", "disconnected", "reconnected", "existing"]);

export const PlayerStatus = z.object({
    player: z.object({
        username: z.string(),
        alive: z.boolean(),
        color: z.string(),
        hasConnectedPreviously: z.boolean(),
        id: z.string(),
    }),
    status: PlayerConnectionStatus
})

export const GameStateEnum = z.enum(["lobby", "inGame", "meeting", "over"]);

export const GameState = z.object({
    type: GameStateEnum,
    content: z.unknown()
});

export const MeetingReasonEnum = z.enum(["bodyReported", "emergencyMeetingCalled"]);

export const MeetingReason = z.object({
    type: MeetingReasonEnum,
    content: z.unknown()
});

export const BodyReported = z.object({
    corpse: z.string(),
    initiator: z.string()
})

export const EmergencyMeetingCalled = z.object({
    initiator: z.string()
})

export const WinnerEnum = z.enum(["imposters", "crewmates"]);

export const RoleAssignment = z.enum(["imposter", "crewmate"]);

export const SetRole = z.object({
    role: RoleAssignment,
    id: z.string(),
})

export const PlayerDied = z.object({
    killer: z.string(),
})

export const Winner = z.enum(["imposters", "crewmates"]);

export const VotingResults = z.object({
    ejectedPlayer: z.string().nullable()
})

export const InvalidAction = z.string();

