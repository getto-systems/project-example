import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { fieldHelp, fieldHelp_error } from "../../../z_vendor/getto-css/preact/design/form"

import { ValidateBoardState } from "../../../z_vendor/getto-application/board/validate_board/action"

export function takeLongtimeField(label: VNodeContent): VNode {
    return fieldHelp({
        help: [
            html`${label}に時間がかかっています`,
            html`30秒以上かかる場合は何かがおかしいので、<br />
                お手数ですが管理者に連絡お願いします`,
        ],
    })
}

export function validationMessage(validateState: ValidateBoardState): readonly VNode[] {
    switch (validateState) {
        case "initial":
        case "valid":
            return []

        case "invalid":
            return [fieldHelp_error(["正しく入力されていません"])]
    }
}
