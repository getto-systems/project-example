import { env } from "../../y_environment/ui/env"

import { lnir } from "../../common/util/icon/detail/line_icon"
import { icon_home } from "../icon"

import { category, item } from "./common"

import { docs_avail } from "../../avail/docs"

import { MenuContent, MenuPermissionRequired, MenuTreeNode } from "../../common/outline/load/infra"

import { Icon } from "../../common/util/icon/data"

export function docsMenuContent(): MenuContent {
    return {
        key: "docs",
        loadMenuBadge: false,
        menuTree: [
            category("MAIN", nothing, [
                item("ホーム", icon_home, "index.html"),
                item("ドキュメント", files, "docs/index.html"),
                item("プライバシーポリシー", files, "docs/privacy-policy.html"),
            ]),
            ...devDocs(),
        ],
    }
}
function devDocs(): readonly MenuTreeNode[] {
    if (env.isProduction) {
        return []
    }
    return [
        category("開発用", nothing, [
            item("認証・認可", files, "docs/auth.html"),
            item(docs_avail.title, files, "docs/avail.html"),
            item("ドキュメント", files, "docs/docs.html"),
            item("coverage api", files, "coverage/api/index.html"),
            item("coverage ui", files, "coverage/ui/index.html"),
        ]),
    ]
}

const nothing: MenuPermissionRequired = { type: "nothing" }

const files: Icon = lnir(["files-alt"])
