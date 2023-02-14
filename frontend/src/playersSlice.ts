import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { Player } from './InGame/StatusOverview'

interface playersState {
    players: Array<Player>,
    player: string
}

const initialState: playersState = {
    players: [],
    player : ""
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
        state.players.push(action.payload);
        state.player = action.payload.id;
      },
      setPlayerColor: (state, action: PayloadAction<string>) => {
        let currentPlayer = state.players.find(player => player.id === state.player)
        currentPlayer!.color = action.payload;
      },
      setPlayerName: (state, action: PayloadAction<string>) => {
        let currentPlayer = state.players.find(player => player.id === state.player)
        currentPlayer!.name = action.payload;
      }
    }
  })
  
  export const { addPlayer, setPlayerColor, setPlayerName } = playersSlice.actions
  
  export default playersSlice.reducer