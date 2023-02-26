import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { v4 as uuidV4 } from "uuid";
import { PlayerStatus } from '../Messages/fromServer';
import { RootState } from "./store"
import z from "zod";

export type Player = {
    role: string | null;
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
            const existingPlayer = state.players.find(player => player.id === payload.id);
            if (existingPlayer == null) {
                state.players.push({
                    role: null,
                    alive: true,
                    ...payload,
                })
            } else {
                existingPlayer.color = payload.color;
                existingPlayer.username = payload.username;
            }
        },
        setPlayerColor: (state, action: PayloadAction<SetPlayerColorPayload>) => {
            const player = state.players.find(player => player.id === action.payload.id);
            if (player == null) {
                throw new Error("Cannot set player color of null player");
            }
            player.color = action.payload.color;
        }
    }
})

export const { updatePlayerStatus, setPlayerColor } = playersSlice.actions

export default playersSlice.reducer
