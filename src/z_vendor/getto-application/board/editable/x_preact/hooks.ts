import { useApplicationActionWithFallback } from "../../../action/x_preact/hooks"

import { EditableBoardAction } from "../action"

export function useEditableState<T>(
    edit: Readonly<{ editable: EditableBoardAction; data: T }> | undefined,
): Readonly<{ isEditable: true }> | Readonly<{ isEditable: false; data: T }> {
    const editableState = useApplicationActionWithFallback(edit?.editable, { isEditable: true })
    if (!edit || editableState.isEditable) {
        return { isEditable: true }
    }
    return { isEditable: false, data: edit.data }
}
