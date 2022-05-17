import { VNode } from "preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { ObserveBoardState } from "../../../z_vendor/getto-application/board/observe_board/action"

import { button_disabled, button_undo } from "../../../z_vendor/getto-css/preact/design/form"

type Props = Readonly<{
    observeState: ObserveBoardState
    label?: VNodeContent
    onClick: { (e: Event): void }
}>
export function ClearChangesButton({ observeState, label, onClick }: Props): VNode {
    const buttonLabel = label || "入力内容をクリア"

    if (observeState.hasChanged) {
        return button_undo({ label: buttonLabel, onClick })
    } else {
        return button_disabled({ label: buttonLabel })
    }
}
