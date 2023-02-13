import { createSlice, PayloadAction } from '@reduxjs/toolkit'

interface userState {
    user: string,
    webSocket: WebSocket | null
}

const initialState: userState = {
    user:'',
    webSocket: null
}

export const userSlice = createSlice({
    name: 'user',
    initialState,
    reducers: {
      setUser: (state, action: PayloadAction<string>) => {
        // Redux Toolkit allows us to write "mutating" logic in reducers. It
        // doesn't actually mutate the state because it uses the immer library,
        // which detects changes to a "draft state" and produces a brand new
        // immutable state based off those changes
        state.user = action.payload
      }
    }
  })
  
  export const { setUser} = userSlice.actions
  
  export default userSlice.reducer