import { createSlice, PayloadAction } from '@reduxjs/toolkit'

interface ErrorsState {
    error: string | null,
}

const initialState: ErrorsState = {
    error: null
}

export const errorsSlice = createSlice({
    name: 'errors',
    initialState,
    reducers: {
        showErrorMessage: (state, action: PayloadAction<string>) => {
            state.error = action.payload;
        },
        hideErrorMessage: (state, _action: PayloadAction<void>) => {
            state.error = null;
        },
    }
})

export const { showErrorMessage, hideErrorMessage } = errorsSlice.actions

export default errorsSlice.reducer
