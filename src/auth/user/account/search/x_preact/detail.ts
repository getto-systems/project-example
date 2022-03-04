import { h, VNode } from "preact"

import { DetailAuthUserAccountAction } from "../action"
import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"
import { notice_gray } from "../../../../../z_vendor/getto-css/preact/design/highlight"
import { button_cancel } from "../../../../../z_vendor/getto-css/preact/design/form"
import { html } from "htm/preact"
import { BACK_TO_LIST_BUTTON } from "../../../../../core/x_preact/design/table"

type EntryProps = Readonly<{
    detail: DetailAuthUserAccountAction
}>
export function DetailAuthUserAccountEntry(props: EntryProps): VNode {
    return html`
        ${container([h(CloseButtonComponent, props)])}
        ${container([box_grow({ body: notice_gray(["詳細コンテンツをここに"]) })])}
    `
}

type CloseButtonProps = EntryProps
function CloseButtonComponent(props: CloseButtonProps): VNode {
    return box_grow({ body: button_cancel({ label: BACK_TO_LIST_BUTTON, onClick }) })

    function onClick() {
        props.detail.close()
    }
}
