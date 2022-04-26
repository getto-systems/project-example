import { VNode } from "preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { button_cancel } from "../../../z_vendor/getto-css/preact/design/form"

type Props = Readonly<{
    label?: VNodeContent
    onClick: { (e: Event): void }
}>
export function BackToListButton({ label, onClick }: Props): VNode {
    return button_cancel({ label: label || "一覧に戻る", onClick })
}
