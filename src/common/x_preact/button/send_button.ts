import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../vnode"

import { useAtom } from "../../../z_vendor/getto-atom/x_preact/hooks"

import { button_disabled, button_send } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_spinner } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

import { Atom } from "../../../z_vendor/getto-atom/atom"
import { ValidateBoardState } from "../../util/board/validate/action"
import { ObserveBoardState } from "../../util/board/observe/action"

import { Icon } from "../../util/icon/data"
import { ConnectState } from "../../util/connect/data"

export function SendButton({
    label,
    icon,
    connect,
    validate,
    observe,
    onClick,
}: Readonly<{
    label: PreactContent
    icon: Icon
    connect: Atom<ConnectState>
    validate: Atom<ValidateBoardState>
    observe: Atom<ObserveBoardState>
    onClick: (e: Event) => void
}>): PreactNode {
    const connectState = useAtom(connect)
    const validateState = useAtom(validate)
    const observeState = useAtom(observe)

    if (connectState.isConnecting) {
        return button_send({ state: "connect", label: buttonLabel(icon_spinner) })
    }

    if (!validateState.valid) {
        return button_disabled({ label: buttonLabel(icon) })
    }

    return button_send({
        state: observeState.hasChanged ? "confirm" : "normal",
        label: buttonLabel(icon),
        onClick,
    })

    function buttonLabel(icon: Icon): PreactNode {
        return html`${label} ${iconHtml(icon)}`
    }
}
