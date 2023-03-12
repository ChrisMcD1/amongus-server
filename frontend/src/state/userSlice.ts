import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { RootState } from './store'

type UserState = {
    id: string,
}

export function selectCurrentPlayerID(store: RootState) {
    return store.user.id;
}

const initialState: UserState = {
    id: '',
}

export const userSlice = createSlice({
    name: 'user',
    initialState,
    reducers: {
        setUserID: (state, action: PayloadAction<string>) => {
            state.id = action.payload
        },
    }
})


export const { setUserID } = userSlice.actions

export default userSlice.reducer
