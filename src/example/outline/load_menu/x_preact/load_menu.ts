import { h, VNode } from "preact"
import { html } from "htm/preact"

import { remoteCommonErrorReason } from "../../../../z_lib/ui/remote/reason"

import { useApplicationAction } from "../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    appMenu,
    menuBody,
    menuBox,
    menuCategory,
    menuFooter,
    menuItem,
} from "../../../../../ui/vendor/getto-css/preact/layout/app"
import {
    badge_alert,
    notice_alert,
} from "../../../../../ui/vendor/getto-css/preact/design/highlight"

import { poweredBy } from "../../../site"

import { LoadMenuAction, LoadMenuState } from "../action"

import { RepositoryError } from "../../../../z_lib/ui/repository/data"
import { Menu, MenuCategoryNode, MenuItemNode } from "../../kernel/data"
import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"

export const MENU_ID = "menu"

type EntryProps = Readonly<{
    menu: LoadMenuAction
}>
export function LoadMenuEntry({ menu }: EntryProps): VNode {
    return h(LoadMenuComponent, {
        menu,
        state: useApplicationAction(menu),
    })
}

type Props = EntryProps &
    Readonly<{
        state: LoadMenuState
    }>

export function LoadMenuComponent(props: Props): VNode {
    switch (props.state.type) {
        case "initial-menu":
            return menu([content([])])

        case "succeed-to-load":
        case "succeed-to-update":
        case "succeed-to-toggle":
            return menu([content(props.state.menu)])

        case "failed-to-update":
            return menu([menuBox(error(props.state.err)), content(props.state.menu)])

        case "required-to-login":
            return menu([menuBox(requiredToLogin())])

        case "repository-error":
            return menu([menuBox(repositoryError(props.state.err))])
    }

    function menu(content: readonly VNode[]) {
        return appMenu([...content, menuFooter(poweredBy)])
    }

    function content(wholeMenu: Menu): VNode {
        // id="menu" は breadcrumb の href="#menu" と対応
        // mobile レイアウトで menu に移動
        return menuBody(MENU_ID, menuContent(wholeMenu, bareCategory))

        interface CategoryDecorator {
            (content: VNode): VNode
        }
        function bareCategory(content: VNode) {
            return content
        }
        function liCategory(content: VNode) {
            return html`<li>${content}</li>`
        }

        function menuContent(menu: Menu, categoryDecorator: CategoryDecorator): readonly VNode[] {
            return menu.map((node) => {
                switch (node.type) {
                    case "category":
                        return categoryDecorator(menuCategoryContent(node))

                    case "item":
                        return menuItemContent(node)
                }
            })
        }

        function menuCategoryContent(node: MenuCategoryNode) {
            const { label } = node.category

            return menuCategory({
                isExpand: node.isExpand,
                label,
                show,
                hide,
                badge: badge(node.badgeCount),
                children: menuContent(node.children, liCategory),
            })

            function show(event: Event) {
                event.preventDefault()
                props.menu.show(node.path)
            }
            function hide(event: Event) {
                event.preventDefault()
                props.menu.hide(node.path)
            }
        }

        function menuItemContent(node: MenuItemNode) {
            const { label, icon, href } = node.item

            return menuItem({
                isActive: node.isActive,
                href,
                content: html`<i class="${icon}"></i> ${label}`,
                badge: badge(node.badgeCount),
            })
        }
    }
}

function badge(badgeCount: number) {
    if (badgeCount === 0) {
        return EMPTY_CONTENT
    }

    return badge_alert(html`${badgeCount}`)
}

function requiredToLogin(): readonly VNode[] {
    return [notice_alert("認証エラー"), html`<small><p>もう一度ログインしてください</p></small>`]
}
function repositoryError(err: RepositoryError): readonly VNode[] {
    switch (err.type) {
        case "infra-error":
            return [notice_alert("ストレージエラー"), ...errorDetail(err.err)]
    }
}
function error(err: RemoteCommonError): readonly VNode[] {
    return remoteCommonErrorReason(err, (reason) => [
        notice_alert(reason.message),
        ...reason.detail.map((message) => html`<small><p>${message}</p></small>`),
    ])
}
function errorDetail(err: string): readonly VNode[] {
    if (err.length === 0) {
        return []
    }
    return [html`<small><p>詳細: ${err}</p></small>`]
}

const EMPTY_CONTENT = html``