import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../vnode"

import { useAtom } from "../../../z_vendor/getto-atom/x_preact/hooks"

import { fieldHelp, fieldHelp_error } from "../../../z_vendor/getto-css/preact/design/form"
import { VectorButton } from "../../util/board/input/x_preact/vector"

import { VectorAddButton } from "../button/vector_add_button"
import { VectorRemoveButton } from "../button/vector_remove_button"
import { VectorUndoRemoveButton } from "../button/vector_undo_remove_button"

import { Atom } from "../../../z_vendor/getto-atom/atom"
import { ValidateBoardState } from "../../util/board/validate/action"

export function takeLongtimeField(label: PreactContent): PreactNode {
    return fieldHelp({
        help: [
            html`${label}に時間がかかっています`,
            html`30秒以上かかる場合は何かがおかしいので、<br />
                お手数ですが管理者に連絡お願いします`,
        ],
    })
}

export function ValidateBoardMessage(
    props: Readonly<{ state: Atom<ValidateBoardState> }>,
): PreactNode {
    const validateState = useAtom(props.state)

    if (validateState.valid) {
        return html``
    }
    return fieldHelp_error(["正しく入力されていません"])
}

export function vectorButton(): VectorButton {
    return {
        add: ({ onClick }) => VectorAddButton({ onClick }),
        remove: ({ onClick }) => VectorRemoveButton({ onClick }),
        undoRemove: ({ onClick }) => VectorUndoRemoveButton({ onClick }),
    }
}
