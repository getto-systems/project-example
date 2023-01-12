import { VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationState } from "../../../../z_vendor/getto-application/action/x_preact/hooks"

import { VNodeContent } from "../../../x_preact/vnode"

import { mainTitleWithSidebarButton } from "../../../../z_vendor/getto-css/preact/layout/app"
import { notice_alert } from "../../../../z_vendor/getto-css/preact/design/highlight"

import { repositoryErrorReason } from "../../repository/x_error/reason"
import { isSidebarExpand } from "./helper"

import { iconHtml } from "../../icon/x_preact/icon"
import { icon_sidebar_expand, icon_sidebar_fold } from "../../../../x_content/icon"

import { ToggleSidebarAction } from "../action"

type Props = Readonly<{
    sidebar: ToggleSidebarAction
    title: VNodeContent
}>
export function MainTitleWithSidebar(props: Props): VNode {
    const state = useApplicationState(props.sidebar.state)

    return html`${[content(), error()]}`

    function content(): VNode {
        return mainTitleWithSidebarButton({
            title: props.title,
            button: sidebarButton(),
        })
    }

    function sidebarButton(): VNode {
        if (isSidebarExpand(state)) {
            return foldButton()
        } else {
            return expandButton()
        }

        function foldButton(): VNode {
            return html`<a href="#" onClick=${onClick}>${iconHtml(icon_sidebar_fold)}</a>`

            function onClick(e: Event) {
                e.preventDefault()
                props.sidebar.fold()
            }
        }
        function expandButton(): VNode {
            return html`<a href="#" onClick=${onClick}>${iconHtml(icon_sidebar_expand)}</a>`

            function onClick(e: Event) {
                e.preventDefault()
                props.sidebar.expand()
            }
        }
    }

    function error(): VNode {
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
