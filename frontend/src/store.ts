import { configureStore } from '@reduxjs/toolkit';
import colorReducer from './JoinGame/colorSlice';
import userReducer from './JoinGame/userSlice';
import playersReducer from './playersSlice';

const store = configureStore({
    reducer: {
      color: colorReducer,
      user: userReducer,
      players: playersReducer,
    },
  })

  // Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>

export type AppDispatch = typeof store.dispatch
export default store