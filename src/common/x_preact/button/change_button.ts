import { h } from "preact"
import { PreactContent, PreactNode } from "../node"

import { SendButton } from "./send_button"

import { icon_change } from "../../../x_content/icon"

import { Atom } from "../../../z_vendor/getto-atom/atom"
import { ValidateBoardState } from "../../util/board/validate/action"
import { ObserveBoardState } from "../../util/board/observe/action"

import { Icon } from "../../util/icon/data"
import { ConnectState } from "../../util/connect/data"

export function ChangeButton(
    props: Readonly<{
        label?: PreactContent
        icon?: Icon
        connect: Atom<ConnectState>
        validate: Atom<ValidateBoardState>
        observe: Atom<ObserveBoardState>
        onClick: { (e: Event): void }
    }>,
): PreactNode {
    return h(SendButton, {
        ...props,
        label: props.label || "変更",
        icon: props.icon || icon_change,
    })
}
