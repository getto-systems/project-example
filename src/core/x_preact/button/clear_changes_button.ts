import { VNode } from "preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"
import { ValidateBoardState } from "../../../z_vendor/getto-application/board/validate_board/action"

import { button_disabled, button_undo } from "../../../z_vendor/getto-css/preact/design/form"

type Props = Readonly<{
    validateState: ValidateBoardState
    label?: VNodeContent
    onClick: { (e: Event): void }
}>
export function ClearChangesButton({ validateState, label, onClick }: Props): VNode {
    const buttonLabel = label || "入力内容をクリア"

    switch (validateState) {
        case "initial":
            return button_disabled({ label: buttonLabel })

        case "valid":
        case "invalid":
            return button_undo({ label: buttonLabel, onClick })
    }
}
