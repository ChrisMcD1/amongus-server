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
