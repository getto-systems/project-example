import { h, VNode } from "preact"

import { VNodeContent } from "../vnode"

import { SendButton } from "./send_button"

import { icon_change } from "../../../x_content/icon"

import { ValidateBoardState } from "../../../z_vendor/getto-application/board/validate_board/action"
import { ObserveBoardState } from "../../../z_vendor/getto-application/board/observe_board/action"

import { Icon } from "../../util/icon/data"

type Props = Readonly<{
    label?: VNodeContent
    icon?: Icon
    isConnecting: boolean
    validateState: ValidateBoardState
    observeState: ObserveBoardState
    onClick: { (e: Event): void }
}>
export function ChangeButton(props: Props): VNode {
    return h(SendButton, {
        ...props,
        label: props.label || "変更",
        icon: props.icon || icon_change,
    })
}
