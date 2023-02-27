import { z } from "zod";

export type PreZodMessage = {
    type: "ChatMessage" |
    "AssignedID" |
    "PlayerStatus" |
    "GameState" | "PlayerRole" | "PlayerDied" | "SuccessfulKill" | "InvalidAction" | "BodyReported" | "VotingResults" | "GameOver";
    content: any;
}

export const ChatMessage = z.object({
    contents: z.string()
})

export const PlayerConnectionStatus = z.enum(["new", "disconnected", "reconnected", "existing"]);

export const PlayerStatus = z.object({
    username: z.string(),
    color: z.string(),
    id: z.string(),
    status: PlayerConnectionStatus
})

export const GameStateEnum = z.enum(["lobby", "inGame", "reset"]);

export const GameState = z.object({
    state: GameStateEnum,
})

export const RoleAssignment = z.enum(["imposter", "crewmate"]);

export const SetRole = z.object({
    role: RoleAssignment
})

export const PlayerDied = z.object({
    killer: z.string(),
})

export const BodyReported = z.object({
    corpse: z.string(),
    initiator: z.string()
})

export const Winner = z.enum(["imposters", "crewmates"]);

export const VotingResults = z.object({
    ejectedPlayer: z.string().nullable()
})

