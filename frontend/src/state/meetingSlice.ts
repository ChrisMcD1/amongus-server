import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { RootState } from './store'
import z from "zod";
import { BodyReported } from '../Messages/fromServer';

type MeetingState = {
    initiator: string,
    reportedBodyID: string | null,
    purpose: "Emergency" | "BodyReported" | null
}

export function selectMeetingInitiator(store: RootState) {
    return store.players.players.find(player => player.id === store.meeting.initiator);
}

const initialState: MeetingState = {
    initiator: '',
    reportedBodyID: null,
    purpose: null,
}

export const meetingSlice = createSlice({
    name: 'meeting',
    initialState,
    reducers: {
        beginEmergencyMeeting: (state, action: PayloadAction<string>) => {
            state.initiator = action.payload
            state.purpose = "Emergency";
        },
        beginReportedBodyMeeting: (state, action: PayloadAction<z.infer<typeof BodyReported>>) => {
            state.initiator = action.payload.initiator;
            state.reportedBodyID = action.payload.corpse;
            state.purpose = "BodyReported";
        },
    }
})


export const { beginEmergencyMeeting, beginReportedBodyMeeting } = meetingSlice.actions

export default meetingSlice.reducer
