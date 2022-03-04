import { AbstractStatefulApplicationAction, StatefulApplicationAction } from "../../action/action"

export interface EditableBoardAction extends StatefulApplicationAction<EditableBoardState> {
    open(): EditableBoardState
    close(): EditableBoardState
}

export type EditableBoardState = Readonly<{ isEditable: boolean }>

const initialState: EditableBoardState = { isEditable: false }

export function initEditableBoardAction(): EditableBoardAction {
    return new Action()
}

class Action
    extends AbstractStatefulApplicationAction<EditableBoardState>
    implements EditableBoardAction
{
    readonly initialState = initialState

    open(): EditableBoardState {
        return this.post({ isEditable: true })
    }
    close(): EditableBoardState {
        return this.post({ isEditable: false })
    }
}
