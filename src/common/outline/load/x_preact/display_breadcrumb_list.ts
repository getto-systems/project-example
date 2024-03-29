import { html } from "htm/preact"
import { PreactNode } from "../../../x_preact/node"

import {
    mainBreadcrumbLink,
    mainBreadcrumbList,
    mainBreadcrumbSeparator,
} from "../../../../z_vendor/getto-css/preact/layout/app"
import { linky } from "../../../../z_vendor/getto-css/preact/design/highlight"
import { lnir } from "../../../util/icon/detail/line_icon"

import { siteInfo } from "../../../../x_content/site"
import { iconHtml } from "../../../util/icon/x_preact/icon"
import { icon_home } from "../../../../x_content/icon"

import { MENU_ID } from "./display_menu"

import { OutlineBreadcrumbListAction } from "../action"

import { BreadcrumbList, BreadcrumbNode, MenuCategory, MenuItem } from "../data"
import { useAtom } from "../../../../z_vendor/getto-atom/x_preact/hooks"

type Props = Readonly<{
    breadcrumbList: OutlineBreadcrumbListAction
}>
export function DisplayOutlineBreadcrumbList(props: Props): PreactNode {
    const breadcrumbListState = useAtom(props.breadcrumbList.state)
    return mainBreadcrumbList(toContent(breadcrumbListState.list))
}

function toContent(breadcrumbList: BreadcrumbList): readonly PreactNode[] {
    return [top()].concat(breadcrumbList.map((node) => withSeparator(...content(node))))

    function content(node: BreadcrumbNode): [string, PreactNode] {
        switch (node.type) {
            case "category":
                return [node.category.label, category(node.category)]

            case "item":
                return [node.item.href, item(node.item)]
        }
    }

    function withSeparator(key: string, content: PreactNode): PreactNode {
        return html`<span class="noWrap" key=${key}>${SEPARATOR}${content}</span>`
    }
}

function top(): PreactNode {
    // トップリンク href="#menu" は menu の id="menu" と対応
    // mobile レイアウトで menu の位置に移動
    return item({
        label: siteInfo.title,
        icon: icon_home,
        href: `#${MENU_ID}`,
    } as MenuItem)
}
function category({ label }: MenuCategory): PreactNode {
    return linky(label)
}
function item({ label, icon, href }: MenuItem): PreactNode {
    return mainBreadcrumbLink(href, html`${iconHtml(icon)} ${label}`)
}

const SEPARATOR = mainBreadcrumbSeparator(iconHtml(lnir(["chevron-right"])))
