import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../x_preact/common"

import { button_disabled, button_send } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_spinner } from "../../../x_content/icon"
import { iconHtml } from "../../../core/x_preact/design/icon"

import { ValidateBoardState } from "../../../z_vendor/getto-application/board/validate_board/action"
import { ObserveBoardState } from "../../../z_vendor/getto-application/board/observe_board/action"

import { Icon } from "../icon/data"

type Props = Readonly<{
    label: VNodeContent
    icon: Icon
    isConnecting: boolean
    validateState: ValidateBoardState
    observeState?: ObserveBoardState // TODO 必須にしたい
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
            return button_send({ state: buttonState("normal"), label: buttonLabel(icon), onClick })

        case "valid":
            return button_send({ state: buttonState("confirm"), label: buttonLabel(icon), onClick })

        case "invalid":
            return button_disabled({ label: buttonLabel(icon) })
    }

    function buttonLabel(icon: Icon): VNode {
        return html`${label} ${iconHtml(icon)}`
    }
    function buttonState(defaultState: "normal" | "confirm"): "normal" | "confirm" {
        if (observeState === undefined) {
            return defaultState
        }
        return observeState.hasChanged ? "confirm" : "normal"
    }
}
