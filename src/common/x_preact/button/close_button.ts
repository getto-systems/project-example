import { PreactContent, PreactNode } from "../node"

import { button_cancel } from "../../../z_vendor/getto-css/preact/design/form"

export function CloseButton({
    label,
    onClick,
}: Readonly<{
    label?: PreactContent
    onClick: { (e: Event): void }
}>): PreactNode {
    return button_cancel({ label: label || "閉じる", onClick })
}
