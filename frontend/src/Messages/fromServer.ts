import { z } from "zod";

export type PreZodMessage = {
    type: "ChatMessage" |
    "AssignedID" |
    "PlayerStatus" |
    "GameState" |
    "PlayerRole" |
    "PlayerDied" |
    "SuccessfulKill" |
    "InvalidAction" |
    "BodyReported" |
    "EmergencyMeetingCalled" |
    "VotingResults" |
    "GameOver";
    content: any;
}

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

export const GameStateEnum = z.enum(["lobby", "inGame", "reset"]);

export const GameState = z.object({
    state: GameStateEnum,
})

export const RoleAssignment = z.enum(["imposter", "crewmate"]);

export const SetRole = z.object({
    role: RoleAssignment,
    id: z.string(),
})

export const PlayerDied = z.object({
    killer: z.string(),
})

export const BodyReported = z.object({
    corpse: z.string(),
    initiator: z.string()
})

export const EmergencyMeetingCalled = z.object({
    initiator: z.string()
})

export const Winner = z.enum(["imposters", "crewmates"]);

export const VotingResults = z.object({
    ejectedPlayer: z.string().nullable()
})

export const InvalidAction = z.string();

