import { html } from "htm/preact"
import { VNode } from "preact"
import { useEffect, useState } from "preact/hooks"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { button_send } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_ok } from "../../../x_content/icon"
import { iconHtml } from "./icon"

export function SuccessButton({
    isSuccess,
    label,
    onClick,
}: Readonly<{
    isSuccess: boolean
    label: VNodeContent
    onClick: { (e: Event): void }
}>): VNode {
    type SuccessButtonState = "success" | "normal"

    const [state, setState] = useState<SuccessButtonState>(isSuccess ? "success" : "normal")

    useEffect(() => {
        if (state === "success") {
            setTimeout(() => {
                setState("normal")
            }, 1000)
        }
    }, [state])

    switch (state) {
        case "success":
            return button_send({
                state: "normal",
                label: html`${label} ${iconHtml(icon_ok)}`,
                onClick,
            })

        case "normal":
            return button_send({ state: "normal", label, onClick })
    }
}
