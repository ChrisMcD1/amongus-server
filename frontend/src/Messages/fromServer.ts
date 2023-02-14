import { z } from "zod";

export type PreZodMessage = {
    type: "PlayerStatus"; // TODO: This string literal will grow as we add more
    content: any;
}

export const PlayerConnectionStatus = z.enum(["new", "disconnected", "reconnected", "existing"]);

export const PlayerStatus = z.object({
    username: z.string(),
    color: z.string(),
    id: z.string(),
    status: PlayerConnectionStatus
})

export const VotingResults = z.object({
    ejectedPlayer: z.string().nullable()
})

export const BodyReported = z.object({
    corpse: z.string(),
    initiator: z.string(),
})

export const PlayerDied = z.object({
    killer: z.string(),
})

export const ChatMessage = z.object({
    contents: z.string(),
})

export const GameStateEnum = z.enum(["lobby", "inGame"]);

export const GameState = z.object({
    state: z.string(),
})

