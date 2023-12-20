import { PreactContent, PreactNode } from "../vnode"

import { useAtom } from "../../../z_vendor/getto-atom/x_preact/hooks"

import { button_disabled, button_undo } from "../../../z_vendor/getto-css/preact/design/form"

import { Atom } from "../../../z_vendor/getto-atom/atom"
import { ObserveBoardState } from "../../util/board/observe/action"

export function ClearChangesButton({
    observe,
    label,
    onClick,
}: Readonly<{
    label?: PreactContent
    observe: Atom<ObserveBoardState>
    onClick: { (e: Event): void }
}>): PreactNode {
    const observeState = useAtom(observe)
    const buttonLabel = label || "入力内容をクリア"

    if (observeState.hasChanged) {
        return button_undo({ label: buttonLabel, onClick })
    } else {
        return button_disabled({ label: buttonLabel })
    }
}
