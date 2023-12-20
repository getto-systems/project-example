import { h } from "preact"
import { PreactContent, PreactNode } from "../../../../../../common/x_preact/vnode"

import { useAtom } from "../../../../../../z_vendor/getto-atom/x_preact/hooks"
import { useEditableState } from "../../../../../../common/util/board/editable/x_preact/hooks"

import { inputField, label } from "../../../../../../z_vendor/getto-css/preact/design/form"
import { checkboxOptions } from "../../../../../../common/x_preact/design/checkbox"
import { CheckboxBoard } from "../../../../../../common/util/board/input/x_preact/checkbox"
import {
    authPermissionCheckboxContent,
    authPermissionGranted,
} from "../../../../account/kernel/x_preact/field"

import { EditableBoardAction } from "../../../../../../common/util/board/editable/action"
import { AuthPermissionGrantedField } from "../action"

import { AuthPermission } from "../../../data"
import { AUTH_USER_ACCOUNT } from "../../../../account/kernel/data"

export function AuthPermissionGrantedField(
    props: Readonly<{ field: AuthPermissionGrantedField }> &
        Partial<{
            title: PreactContent
            help: readonly PreactContent[]
            edit: Readonly<{
                data: Readonly<{ granted: readonly AuthPermission[] }>
                editable: EditableBoardAction
            }>
        }>,
): PreactNode {
    const editableState = useEditableState(props.edit)
    const options = useAtom(props.field.options)

    return inputField({
        title: props.title || AUTH_USER_ACCOUNT["granted"],
        help: props.help,
        label: label,
        editableState,
        body: editableState.isEditable
            ? h(CheckboxBoard, {
                  input: props.field.input,
                  options: checkboxOptions(options, authPermissionCheckboxContent),
              })
            : authPermissionGranted(editableState.data),
    })
}
