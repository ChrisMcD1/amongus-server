import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { v4 as uuidV4 } from "uuid";
import { PlayerStatus, RoleAssignment } from '../Messages/fromServer';
import { RootState } from "./store"
import z from "zod";

export type Player = {
    role: z.infer<typeof RoleAssignment> | null;
    color: string;
    username: string;
    alive: boolean;
    id: ReturnType<typeof uuidV4>;
};

interface PlayersState {
    players: Array<Player>,
}

const initialState: PlayersState = {
    players: [],
}

export function selectOtherPlayers(store: RootState) {
    return store.players.players.filter(player => player.id !== store.user.id);
}

export function selectCurrentPlayer(store: RootState) {
    return store.players.players.find(player => player.id === store.user.id);
}


export type SetPlayerRolePayload = {
    role: z.infer<typeof RoleAssignment>,
    id: string
}

export type SetPlayerColorPayload = {
    color: string,
    id: string
}

export const playersSlice = createSlice({
    name: 'user',
    initialState,
    reducers: {
        updatePlayerStatus: (state, action: PayloadAction<z.infer<typeof PlayerStatus>>) => {
            const payload = action.payload;
            const existingPlayer = state.players.find(player => player.id === payload.player.id);
            if (existingPlayer == null) {
                state.players.push({
                    role: null,
                    ...payload.player,
                })
            } else {
                for (const [key, value] of Object.entries(payload.player)) {
                    if (key in existingPlayer) {
                        //@ts-ignore
                        existingPlayer[key] = value;
                    }
                }
            }
        },
        setPlayerDead: (state, action: PayloadAction<string>) => {
            let targetPlayer = state.players.find(player => player.id === action.payload);
            if (targetPlayer == null) {
                console.error(`Tried to set ${action.payload} dead, but they do not exist!`);
                return;
            }
            targetPlayer.alive = false;
        },
        setPlayerRole: (state, action: PayloadAction<SetPlayerRolePayload>) => {
            const player = state.players.find(player => player.id === action.payload.id);
            if (player == null) {
                throw new Error("Cannot set player role of null player");
            }
            player.role = action.payload.role;
        },
        setPlayerColor: (state, action: PayloadAction<SetPlayerColorPayload>) => {
            const player = state.players.find(player => player.id === action.payload.id);
            if (player == null) {
                throw new Error("Cannot set player color of null player");
            }
            player.color = action.payload.color;
        },
        deleteAllPlayers: (state, _action: PayloadAction<void>) => {
            state.players = [];
        },
    }
})

export const { updatePlayerStatus, setPlayerColor, deleteAllPlayers, setPlayerRole, setPlayerDead } = playersSlice.actions

export default playersSlice.reducer
