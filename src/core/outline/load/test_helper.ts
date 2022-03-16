import { Icon } from "../../../z_lib/ui/icon/data"
import { MenuCategoryLabel } from "./data"
import { MenuPermission, MenuTree, MenuTreeNode } from "./infra"

export function standard_MenuTree(): MenuTree {
    const allow: MenuPermission = { type: "allow" }
    const user: MenuPermission = { type: "role", role: "user" }

    return [
        category("MAIN", allow, [
            item("ホーム", icon("home"), "index.html"),
            item("ドキュメント", icon("docs"), "docs/index.html"),
        ]),
        category("DOCUMENT", allow, [
            item("認証・認可", icon("auth"), "docs/auth.html"),
            category("DETAIL", allow, [item("詳細", icon("detail"), "docs/auth.html")]),
        ]),
        category("ACCOUNT", user, [
            item("ユーザー", icon("friends"), "user/account.html"),
        ]),
    ]
}

function category(label: string, permission: MenuPermission, children: MenuTree): MenuTreeNode {
    return { type: "category", category: { label, permission }, children }
}
function item(label: string, icon: Icon, path: string): MenuTreeNode {
    return { type: "item", item: { label, icon, path } }
}

function icon(icon: string): Icon {
    return icon as unknown as Icon
}

export function markMenuCategoryLabel(label: string): MenuCategoryLabel {
    return label as MenuCategoryLabel
}
