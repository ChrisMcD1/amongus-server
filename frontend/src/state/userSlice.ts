import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { RootState } from './store'

interface UserState {
    id: string,
    webSocket: WebSocket | null
}

export function selectCurrentPlayerID(store: RootState) {
    return store.user.id;
}

const initialState: UserState = {
    id: '',
    webSocket: null
}

export const userSlice = createSlice({
    name: 'user',
    initialState,
    reducers: {
        setUserID: (state, action: PayloadAction<string>) => {
            state.id = action.payload
        }
    }
})

export const { setUserID } = userSlice.actions

export default userSlice.reducer
