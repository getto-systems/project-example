import { VNode } from "preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { button_undo } from "../../../z_vendor/getto-css/preact/design/form"

type Props = Readonly<{
    label?: VNodeContent
    onClick: { (e: Event): void }
}>
export function ClearSearchButton({ label, onClick }: Props): VNode {
    const buttonLabel = label || "検索項目をクリア"
    return button_undo({ label: buttonLabel, onClick })
}
