import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../vnode"

import { button_disabled, button_send } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_spinner } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

import { ValidateBoardState } from "../../../z_vendor/getto-application/board/validate_board/action"
import { ObserveBoardState } from "../../../z_vendor/getto-application/board/observe_board/action"

import { Icon } from "../../util/icon/data"

type Props = Readonly<{
    label: VNodeContent
    icon: Icon
    isConnecting: boolean
    validateState: ValidateBoardState
    observeState: ObserveBoardState
    onClick: { (e: Event): void }
}>
export function SendButton({
    isConnecting,
    validateState,
    observeState,
    label,
    icon,
    onClick,
}: Props): VNode {
    if (isConnecting) {
        return button_send({ state: "connect", label: buttonLabel(icon_spinner) })
    }

    switch (validateState) {
        case "initial":
        case "valid":
            return button_send({ state: buttonState(), label: buttonLabel(icon), onClick })

        case "invalid":
            return button_disabled({ label: buttonLabel(icon) })
    }

    function buttonLabel(icon: Icon): VNode {
        return html`${label} ${iconHtml(icon)}`
    }
    function buttonState(): "normal" | "confirm" {
        return observeState.hasChanged ? "confirm" : "normal"
    }
}
