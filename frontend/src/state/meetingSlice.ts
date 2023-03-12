import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { RootState } from './store'

type MeetingState = {
    initiator: string,
}

export function selectMeetingInitiator(store: RootState) {
    return store.players.players.find(player => player.id === store.meeting.initiator);
}

const initialState: MeetingState = {
    initiator: '',
}

export const meetingSlice = createSlice({
    name: 'meeting',
    initialState,
    reducers: {
        beginEmergencyMeeting: (state, action: PayloadAction<string>) => {
            state.initiator = action.payload
        },
    }
})


export const { beginEmergencyMeeting } = meetingSlice.actions

export default meetingSlice.reducer
