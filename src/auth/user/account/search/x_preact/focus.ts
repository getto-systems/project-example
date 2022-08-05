import { h, VNode } from "preact"
import { html } from "htm/preact"

import { box_grow, container } from "../../../../../z_vendor/getto-css/preact/design/box"
import { notice_gray } from "../../../../../z_vendor/getto-css/preact/design/highlight"

import { DetailAuthUserAccount, DetailAuthUserAccountActions } from "../../kernel/x_preact/detail"
import { BackToListButton } from "../../../../../common/x_preact/button/back_to_list_button"

import { SearchAuthUserAccountAction } from "../action"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

type Props = DetailAuthUserAccountActions &
    Readonly<{
        search: SearchAuthUserAccountAction
    }>
export function FocusAuthUserAccount(props: Props): VNode {
    const focusState = useApplicationState(props.search.list.focus.state)

    return html`${[container([box_grow({ body: backToListButton() })]), content()]}`

    function content(): VNode {
        if (focusState.type === "detect-failed") {
            return container([
                box_grow({ body: notice_gray(["指定されたユーザーが見つかりませんでした"]) }),
            ])
        }

        return h(DetailAuthUserAccount, props)
    }

    function backToListButton(): VNode {
        return h(BackToListButton, { onClick })

        function onClick() {
            props.search.list.focus.close()
        }
    }
}
