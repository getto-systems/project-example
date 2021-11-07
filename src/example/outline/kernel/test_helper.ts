import { MenuCategoryLabel } from "./data"
import { MenuPermission, MenuTree, MenuTreeNode } from "./infra"

export function standard_MenuTree(): MenuTree {
    const allow: MenuPermission = { type: "allow" }
    const dev: MenuPermission = { type: "role", role: "dev-docs" }

    return [
        category("MAIN", allow, [
            item("ホーム", "home", "index.html"),
            item("ドキュメント", "docs", "docs/index.html"),
        ]),
        category("DOCUMENT", allow, [
            item("認証・認可", "auth", "docs/auth.html"),
            category("DETAIL", allow, [item("詳細", "detail", "docs/auth.html")]),
        ]),
        category("DEVELOPMENT", dev, [
            item("配備構成", "deployment", "docs/z-dev/deployment.html"),
        ]),
    ]
}

function category(label: string, permission: MenuPermission, children: MenuTree): MenuTreeNode {
    return { type: "category", category: { label, permission }, children }
}
function item(label: string, icon: string, path: string): MenuTreeNode {
    return { type: "item", item: { label, icon, path } }
}

export function markMenuCategoryLabel(label: string): MenuCategoryLabel {
    return label as MenuCategoryLabel
}
