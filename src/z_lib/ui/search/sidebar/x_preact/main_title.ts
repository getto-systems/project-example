import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import { VNodeContent } from "../../../x_preact/common"

import { mainTitleWithSidebarButton } from "../../../../../z_vendor/getto-css/preact/layout/app"

import { SearchSidebarAction, SearchSidebarState } from "../action"
import { repositoryErrorReason } from "../../../repository/x_error/reason"
import { notice_alert } from "../../../../../z_vendor/getto-css/preact/design/highlight"
import { iconHtml } from "../../../../../core/x_preact/design/icon"
import { icon_sidebar_expand, icon_sidebar_fold } from "../../../../../x_content/icon"
import { sidebarExpand } from "./helper"

type EntryProps = Readonly<{
    sidebar: SearchSidebarAction
    title: VNodeContent
}>
export function MainTitleWithSidebarEntry(props: EntryProps): VNode {
    return h(MainTitleWithSidebarComponent, {
        ...props,
        state: useApplicationAction(props.sidebar),
    })
}

type Props = EntryProps &
    Readonly<{
        state: SearchSidebarState
    }>
export function MainTitleWithSidebarComponent(props: Props): VNode {
    return html`${content()}${h(SearchSidebarError, props)}`

    function content(): VNode {
        return mainTitleWithSidebarButton({
            title: props.title,
            button: h(SearchSidebarButton, props),
        })
    }
}

function SearchSidebarButton(props: Props): VNode {
    if (sidebarExpand(props.state)) {
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

function SearchSidebarError({ state }: Readonly<{ state: SearchSidebarState }>): VNode {
    switch (state.type) {
        case "success":
            return EMPTY_CONTENT

        case "repository-error":
            return notice_alert(
                repositoryErrorReason(state.err, (reason) => [
                    `${reason.message}によりサイドバー開閉に失敗しました`,
                    ...reason.detail,
                ]),
            )
    }
}

const EMPTY_CONTENT = html``
