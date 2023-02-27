import { combineReducers, configureStore, getDefaultMiddleware } from '@reduxjs/toolkit';
import userReducer from './userSlice';
import playersReducer from './playersSlice';
import { createReduxHistoryContext } from 'redux-first-history';
import { createBrowserHistory } from 'history';

const { createReduxHistory, routerMiddleware, routerReducer } = createReduxHistoryContext({
    history: createBrowserHistory(),
})

export const store = configureStore({
    reducer: combineReducers({
        router: routerReducer,
        user: userReducer,
        players: playersReducer,
    }),
    middleware: (getDefaultMiddleware) => getDefaultMiddleware().concat(routerMiddleware),
})

export const history = createReduxHistory(store);

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>

export type AppDispatch = typeof store.dispatch
export default store
