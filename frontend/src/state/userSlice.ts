import { createSlice, PayloadAction } from '@reduxjs/toolkit'

interface userState {
    id: string,
    webSocket: WebSocket | null
}

const initialState: userState = {
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
