import { PreactContent, PreactNode } from "../node"

import { useAtom } from "../../../z_vendor/getto-atom/x_preact/hooks"

import { Atom } from "../../../z_vendor/getto-atom/atom"
import { ObserveBoardState } from "../../util/board/observe/action"

import { button_disabled, button_undo } from "../../../z_vendor/getto-css/preact/design/form"

export function ResetButton({
    observe,
    label,
    onClick,
}: Readonly<{
    label?: PreactContent
    observe: Atom<ObserveBoardState>
    onClick: { (e: Event): void }
}>): PreactNode {
    const observeState = useAtom(observe)

    const buttonLabel = label || "変更前に戻す"

    if (observeState.hasChanged) {
        return button_undo({ label: buttonLabel, onClick })
    } else {
        return button_disabled({ label: buttonLabel })
    }
}
