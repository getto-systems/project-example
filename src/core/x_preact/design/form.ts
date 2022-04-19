import { html } from "htm/preact"
import { VNode } from "preact"

import { VNodeContent } from "../../../z_lib/ui/x_preact/common"

import { fieldHelp } from "../../../z_vendor/getto-css/preact/design/form"

import { icon_spinner } from "../../../x_content/icon"
import { iconHtml } from "./icon"

export function takeLongtimeField(label: VNodeContent): VNode {
    return fieldHelp({
        help: [
            html`${iconHtml(icon_spinner)} ${label}に時間がかかっています`,
            html`30秒以上かかる場合は何かがおかしいので、お手数ですが管理者に連絡お願いします`,
        ],
    })
}
