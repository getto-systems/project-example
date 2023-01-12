import { VNode } from "preact"

import { VNodeContent } from "../vnode"

import { button_cancel } from "../../../z_vendor/getto-css/preact/design/form"

type Props = Readonly<{
    label?: VNodeContent
    onClick: { (e: Event): void }
}>
export function BackToRegisterButton({ label, onClick }: Props): VNode {
    return button_cancel({ label: label || "登録に戻る", onClick })
}
