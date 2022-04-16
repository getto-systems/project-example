import { html } from "htm/preact"
import { VNode } from "preact"
import { useEffect, useState } from "preact/hooks"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { button_edit } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_ok } from "../../../x_content/icon"
import { iconHtml } from "../design/icon"

type Props = Readonly<{
    isSuccess: boolean
    label?: VNodeContent
    onClick: { (e: Event): void }
}>
export function EditButton({ isSuccess, label, onClick }: Props): VNode {
    type SuccessButtonState = "success" | "normal"

    const [state, setState] = useState<SuccessButtonState>(isSuccess ? "success" : "normal")

    useEffect(() => {
        if (state === "success") {
            setTimeout(() => {
                setState("normal")
            }, 1000)
        }
    }, [state])

    const buttonLabel = label || "変更"

    switch (state) {
        case "success":
            return button_edit({
                state: "normal",
                label: html`${buttonLabel} ${iconHtml(icon_ok)}`,
                onClick,
            })

        case "normal":
            return button_edit({ state: "normal", label: buttonLabel, onClick })
    }
}
