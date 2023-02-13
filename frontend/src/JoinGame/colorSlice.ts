import { createSlice, PayloadAction } from '@reduxjs/toolkit'

export const colorSlice = createSlice({
    name: 'color',
    initialState: {
      color:'#FF0000'
    },
    reducers: {
      changeColor: (state, action: PayloadAction<string>) => {
        // Redux Toolkit allows us to write "mutating" logic in reducers. It
        // doesn't actually mutate the state because it uses the immer library,
        // which detects changes to a "draft state" and produces a brand new
        // immutable state based off those changes
        state.color = action.payload
      },
      
    }
  })
  
  export const { changeColor } = colorSlice.actions
  
  export default colorSlice.reducer