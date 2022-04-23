import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { useSuccessState } from "./hooks"

import { button_edit } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_edit, icon_ok } from "../../../x_content/icon"
import { iconHtml } from "../design/icon"

type Props = Readonly<{
    isSuccess: boolean
    label?: VNodeContent
    onClick: { (e: Event): void }
}>
export function EditButton({ isSuccess, label, onClick }: Props): VNode {
    const state = useSuccessState(isSuccess)

    return button_edit({
        state: "normal",
        label: html`${label || "変更"} ${iconHtml(buttonIcon())}`,
        onClick,
    })

    function buttonIcon() {
        switch (state) {
            case "success-confirming":
                return icon_ok

            case "normal":
                return icon_edit
        }
    }
}
