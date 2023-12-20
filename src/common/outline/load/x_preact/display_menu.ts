import { html } from "htm/preact"
import { PreactNode } from "../../../x_preact/vnode"

import { remoteCommonErrorReason } from "../../../util/remote/x_error/reason"

import { useAtom } from "../../../../z_vendor/getto-atom/x_preact/hooks"

import {
    appMenu,
    menuBody,
    menuBox,
    menuCategory,
    menuFooter,
    menuItem,
} from "../../../../z_vendor/getto-css/preact/layout/app"
import { badge_alert, notice_alert } from "../../../../z_vendor/getto-css/preact/design/highlight"

import { poweredBy } from "../../../../x_content/site"
import { iconHtml } from "../../../util/icon/x_preact/icon"

import { OutlineMenuAction } from "../action"

import { RepositoryError } from "../../../util/repository/data"
import { RemoteCommonError } from "../../../util/remote/data"
import { Menu, MenuCategoryNode, MenuItemNode } from "../data"

export const MENU_ID = "menu"

type Props = Readonly<{
    menu: OutlineMenuAction
}>
export function DisplayOutlineMenu(props: Props): PreactNode {
    const state = useAtom(props.menu.state)

    switch (state.type) {
        case "initial-menu":
            return menu([content([])])

        case "succeed-to-load":
        case "succeed-to-update":
        case "succeed-to-toggle":
            return menu([content(state.menu)])

        case "failed-to-update":
            return menu([menuBox(error(state.err)), content(state.menu)])

        case "required-to-login":
            return menu([menuBox(requiredToLogin())])

        case "repository-error":
            return menu([menuBox(repositoryError(state.err))])
    }

    function menu(content: readonly PreactNode[]) {
        return appMenu([...content, menuFooter(poweredBy)])
    }

    function content(wholeMenu: Menu): PreactNode {
        // id="menu" は breadcrumb の href="#menu" と対応
        // mobile レイアウトで menu に移動
        return menuBody(MENU_ID, menuContent(wholeMenu, bareCategory))

        interface CategoryDecorator {
            (content: PreactNode): PreactNode
        }
        function bareCategory(content: PreactNode) {
            return content
        }
        function liCategory(content: PreactNode) {
            return html`<li>${content}</li>`
        }

        function menuContent(
            menu: Menu,
            categoryDecorator: CategoryDecorator,
        ): readonly PreactNode[] {
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
                content: html`${iconHtml(icon)} ${label}`,
                badge: badge(node.badgeCount),
            })
        }
    }
}

function badge(badgeCount: number) {
    if (badgeCount === 0) {
        return html``
    }

    return badge_alert(html`${badgeCount}`)
}

function requiredToLogin(): readonly PreactNode[] {
    return [notice_alert("認証エラー"), html`<small><p>もう一度ログインしてください</p></small>`]
}
function repositoryError(err: RepositoryError): readonly PreactNode[] {
    switch (err.type) {
        case "infra-error":
            return [notice_alert("ストレージエラー"), ...errorDetail(err.err)]
    }
}
function error(err: RemoteCommonError): readonly PreactNode[] {
    return remoteCommonErrorReason(err, (reason) => [
        notice_alert(reason.message),
        ...reason.detail.map((message) => html`<small><p>${message}</p></small>`),
    ])
}
function errorDetail(err: string): readonly PreactNode[] {
    if (err.length === 0) {
        return []
    }
    return [html`<small><p>詳細: ${err}</p></small>`]
}
