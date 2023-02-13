import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { Player } from './InGame/StatusOverview'

interface playersState {
    players: Array<Player>
}

const initialState: playersState = {
    players: []
}

export const playersSlice = createSlice({
    name: 'user',
    initialState,
    reducers: {
      addPlayer: (state, action: PayloadAction<Player>) => {
        // Redux Toolkit allows us to write "mutating" logic in reducers. It
        // doesn't actually mutate the state because it uses the immer library,
        // which detects changes to a "draft state" and produces a brand new
        // immutable state based off those changes
        state.players.push(action.payload)
      }
    }
  })
  
  export const { addPlayer } = playersSlice.actions
  
  export default playersSlice.reducer