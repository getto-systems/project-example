import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { button_disabled, button_send } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_add, icon_ok, icon_spinner } from "../../../x_content/icon"
import { iconHtml } from "../design/icon"

import { ValidateBoardState } from "../../../z_vendor/getto-application/board/validate_board/action"
import { ObserveBoardState } from "../../../z_vendor/getto-application/board/observe_board/action"

import { Icon } from "../../../z_lib/ui/icon/data"
import { useSuccessState } from "./hooks"

type Props = Readonly<{
    label?: VNodeContent
    icon?: Icon
    isSuccess: boolean
    isConnecting: boolean
    validateState: ValidateBoardState
    observeState: ObserveBoardState
    onClick: { (e: Event): void }
}>
export function RegisterButton({
    isSuccess,
    isConnecting,
    validateState,
    observeState,
    label,
    icon,
    onClick,
}: Props): VNode {
    const state = useSuccessState(isSuccess)

    const buttonIcon = icon || icon_add

    if (state === "success-confirming") {
        return button_send({ state: buttonState(), label: buttonLabel(icon_ok), onClick })
    }

    if (isConnecting) {
        return button_send({ state: "connect", label: buttonLabel(icon_spinner) })
    }

    switch (validateState) {
        case "initial":
            return button_send({ state: buttonState(), label: buttonLabel(buttonIcon), onClick })

        case "valid":
            return button_send({ state: buttonState(), label: buttonLabel(buttonIcon), onClick })

        case "invalid":
            return button_disabled({ label: buttonLabel(buttonIcon) })
    }

    function buttonLabel(icon: Icon): VNode {
        return html`${label || "登録"} ${iconHtml(icon)}`
    }
    function buttonState(): "normal" | "confirm" {
        return observeState.hasChanged ? "confirm" : "normal"
    }
}
