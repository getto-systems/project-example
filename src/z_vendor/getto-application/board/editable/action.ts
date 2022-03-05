import { AbstractStatefulApplicationAction, StatefulApplicationAction } from "../../action/action"

export interface EditableBoardAction extends StatefulApplicationAction<EditableBoardState> {
    open(): EditableBoardState
    close(): EditableBoardState
}

export type EditableBoardState = Readonly<{ isEditable: boolean }>

const initialState: EditableBoardState = { isEditable: false }

export function initEditableBoardAction(
    defaultState: EditableBoardState = initialState,
): EditableBoardAction {
    return new Action(defaultState)
}

class Action
    extends AbstractStatefulApplicationAction<EditableBoardState>
    implements EditableBoardAction
{
    initialState: EditableBoardState

    constructor(defaultState: EditableBoardState) {
        super()
        this.initialState = defaultState
    }

    open(): EditableBoardState {
        return this.post({ isEditable: true })
    }
    close(): EditableBoardState {
        return this.post({ isEditable: false })
    }
}
