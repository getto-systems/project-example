import { initApplicationStateAction, StatefulApplicationAction } from "../../action/action"

export interface EditableBoardAction extends StatefulApplicationAction<EditableBoardState> {
    open(): EditableBoardState
    close(): EditableBoardState
}

export type EditableBoardState = Readonly<{ isEditable: boolean }>

const initialState: EditableBoardState = { isEditable: false }

export function initEditableBoardAction(
    defaultState: EditableBoardState = initialState,
): EditableBoardAction {
    const { state, post } = initApplicationStateAction({ initialState: defaultState })
    return {
        state,
        open(): EditableBoardState {
            return post({ isEditable: true })
        },
        close(): EditableBoardState {
            return post({ isEditable: false })
        },
    }
}
