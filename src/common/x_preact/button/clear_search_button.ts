import { PreactContent, PreactNode } from "../vnode"

import { button_undo } from "../../../z_vendor/getto-css/preact/design/form"

export function ClearSearchButton({
    label,
    onClick,
}: Readonly<{
    label?: PreactContent
    onClick: { (e: Event): void }
}>): PreactNode {
    const buttonLabel = label || "検索項目をクリア"
    return button_undo({ label: buttonLabel, onClick })
}
