import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../vnode"

import { button_disabled, button_send } from "../../../z_vendor/getto-css/preact/design/form"
import { icon_add, icon_ok, icon_spinner } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

import { Atom } from "../../../z_vendor/getto-atom/atom"
import { ValidateBoardState } from "../../util/board/validate/action"
import { ObserveBoardState } from "../../util/board/observe/action"

import { Icon } from "../../util/icon/data"
import { ConnectState, SuccessState } from "../../util/connect/data"
import { useAtom } from "../../../z_vendor/getto-atom/x_preact/hooks"

export function RegisterButton({
    label,
    icon,
    successIcon,
    success,
    connect,
    validate,
    observe,
    onClick,
}: Readonly<{
    label?: PreactContent
    icon?: Icon
    successIcon?: Icon
    success: Atom<SuccessState>
    connect: Atom<ConnectState>
    validate: Atom<ValidateBoardState>
    observe: Atom<ObserveBoardState>
    onClick: { (e: Event): void }
}>): PreactNode {
    const successState = useAtom(success)
    const connectState = useAtom(connect)
    const validateState = useAtom(validate)
    const observeState = useAtom(observe)

    if (successState.isSuccess) {
        return button_send({
            state: "normal",
            label: buttonLabel(successIcon || icon_ok),
            onClick,
        })
    }

    const buttonIcon = icon || icon_add

    if (connectState.isConnecting) {
        return button_send({ state: "connect", label: buttonLabel(icon_spinner) })
    }

    if (!validateState.valid) {
        return button_disabled({ label: buttonLabel(buttonIcon) })
    }

    return button_send({
        state: observeState.hasChanged ? "confirm" : "normal",
        label: buttonLabel(buttonIcon),
        onClick,
    })

    function buttonLabel(icon: Icon): PreactNode {
        return html`${label || "登録"} ${iconHtml(icon)}`
    }
}
