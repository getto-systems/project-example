import { html } from "htm/preact"
import { PreactContent, PreactNode } from "../../../x_preact/node"

import { useAtom } from "../../../../z_vendor/getto-atom/x_preact/hooks"

import { mainTitleWithSidebarButton } from "../../../../z_vendor/getto-css/preact/layout/app"
import { notice_alert } from "../../../../z_vendor/getto-css/preact/design/highlight"

import { repositoryErrorReason } from "../../repository/x_error/reason"
import { isSidebarExpand } from "./helper"

import { iconHtml } from "../../icon/x_preact/icon"
import { icon_sidebar_expand, icon_sidebar_fold } from "../../../../x_content/icon"

import { ToggleSidebarAction } from "../action"

type Props = Readonly<{
    sidebar: ToggleSidebarAction
    title: PreactContent
}>
export function MainTitleWithSidebar(props: Props): PreactNode {
    const state = useAtom(props.sidebar.state)

    return html`${[content(), error()]}`

    function content(): PreactNode {
        return mainTitleWithSidebarButton({
            title: props.title,
            button: sidebarButton(),
        })
    }

    function sidebarButton(): PreactNode {
        if (isSidebarExpand(state)) {
            return foldButton()
        } else {
            return expandButton()
        }

        function foldButton(): PreactNode {
            return html`<a href="#" onClick=${onClick}>${iconHtml(icon_sidebar_fold)}</a>`

            function onClick(e: Event) {
                e.preventDefault()
                props.sidebar.fold()
            }
        }
        function expandButton(): PreactNode {
            return html`<a href="#" onClick=${onClick}>${iconHtml(icon_sidebar_expand)}</a>`

            function onClick(e: Event) {
                e.preventDefault()
                props.sidebar.expand()
            }
        }
    }

    function error(): PreactNode {
        switch (state.type) {
            case "success":
                return html``

            case "repository-error":
                return notice_alert(
                    repositoryErrorReason(state.err, (reason) => [
                        `${reason.message}によりサイドバー開閉に失敗しました`,
                        ...reason.detail,
                    ]),
                )
        }
    }
}
