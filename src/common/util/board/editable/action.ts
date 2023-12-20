import { Atom, initAtom } from "../../../../z_vendor/getto-atom/atom"

export interface EditableBoardAction {
    readonly state: Atom<EditableBoardState>
    open(): EditableBoardState
    close(): EditableBoardState
}

export type EditableBoardState = Readonly<{ isEditable: boolean }>

const initialState: EditableBoardState = { isEditable: false }

export function initEditableBoardAction(
    defaultState: EditableBoardState = initialState,
): EditableBoardAction {
    const { state, post } = initAtom({ initialState: defaultState })
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
