import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../node"

import { useAtom } from "../../../z_vendor/getto-atom/x_preact/hooks"

import { button_delete } from "../../../z_vendor/getto-css/preact/design/form"
import { icon_delete, icon_spinner } from "../../../x_content/icon"
import { iconHtml } from "../../util/icon/x_preact/icon"

import { Atom } from "../../../z_vendor/getto-atom/atom"

import { Icon } from "../../util/icon/data"
import { ConnectState } from "../../util/connect/data"

export function DeleteButton({
    connect,
    label,
    icon,
    onClick,
}: Readonly<{
    label?: PreactContent
    icon?: Icon
    connect: Atom<ConnectState>
    onClick: { (e: Event): void }
}>): PreactNode {
    const connectState = useAtom(connect)

    if (connectState.isConnecting) {
        return button_delete({ state: "connect", label: buttonLabel(icon_spinner) })
    }

    return button_delete({ state: "confirm", label: buttonLabel(icon || icon_delete), onClick })

    function buttonLabel(icon: Icon): PreactNode {
        return html`${label || "削除"} ${iconHtml(icon)}`
    }
}
