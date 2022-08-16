import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { fieldHelp, fieldHelp_error } from "../../../z_vendor/getto-css/preact/design/form"
import { VectorButton } from "../../../z_vendor/getto-application/board/input/x_preact/vector"

import { VectorAddButton } from "../button/vector_add_button"
import { VectorRemoveButton } from "../button/vector_remove_button"
import { VectorUndoRemoveButton } from "../button/vector_undo_remove_button"

import { ValidateBoardState } from "../../../z_vendor/getto-application/board/validate_board/action"
import { ApplicationState } from "../../../z_vendor/getto-application/action/action"
import { useApplicationState } from "../../../z_vendor/getto-application/action/x_preact/hooks"

export function takeLongtimeField(label: VNodeContent): VNode {
    return fieldHelp({
        help: [
            html`${label}に時間がかかっています`,
            html`30秒以上かかる場合は何かがおかしいので、<br />
                お手数ですが管理者に連絡お願いします`,
        ],
    })
}

export function ValidationMessage(
    props: Readonly<{ state: ApplicationState<ValidateBoardState> }>,
): VNode {
    const validateState = useApplicationState(props.state)

    switch (validateState) {
        case "initial":
        case "valid":
            return html``

        case "invalid":
            return fieldHelp_error(["正しく入力されていません"])
    }
}

export function vectorButton(): VectorButton {
    return {
        add: ({ onClick }) => VectorAddButton({ onClick }),
        remove: ({ onClick }) => VectorRemoveButton({ onClick }),
        undoRemove: ({ onClick }) => VectorUndoRemoveButton({ onClick }),
    }
}
