import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../vnode"

import { button_send } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_link, icon_spinner } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

import { Atom } from "../../../z_vendor/getto-atom/atom"

import { Icon } from "../../util/icon/data"
import { ConnectState } from "../../util/connect/data"
import { useAtom } from "../../../z_vendor/getto-atom/x_preact/hooks"

export function LinkButton({
    label,
    icon,
    connect,
    onClick,
}: Readonly<{
    label?: PreactContent
    icon?: Icon
    connect: Atom<ConnectState>
    onClick: { (e: Event): void }
}>): PreactNode {
    const connectState = useAtom(connect)
    const buttonIcon = icon || icon_link

    if (connectState.isConnecting) {
        return button_send({ state: "connect", label: buttonLabel(icon_spinner) })
    }

    return button_send({ state: "normal", label: buttonLabel(buttonIcon), onClick })

    function buttonLabel(icon: Icon): PreactNode {
        return html`${label || "登録"} ${iconHtml(icon)}`
    }
}
