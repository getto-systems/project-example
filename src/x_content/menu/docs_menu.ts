import { env } from "../../y_environment/ui/env"

import { lnir } from "../../z_lib/ui/icon/init/line_icon"
import { icon_home } from "../icon"

import { category, item } from "./common"

import { docs_avail } from "../../avail/docs"

import { MenuContent, MenuPermission, MenuTreeNode } from "../../core/outline/load/infra"

import { Icon } from "../../z_lib/ui/icon/data"

export function docsMenuContent(): MenuContent {
    return {
        key: "docs",
        loadMenuBadge: false,
        menuTree: [
            category("MAIN", allow, [
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
        category("開発用", allow, [
            item("認証・認可", files, "docs/auth.html"),
            item(docs_avail.title, files, "docs/avail.html"),
            item("ドキュメント", files, "docs/docs.html"),
            item("coverage api", files, "coverage/api/index.html"),
            item("coverage ui", files, "coverage/ui/lcov-report/index.html"),
        ]),
    ]
}

const allow: MenuPermission = { type: "allow" }

const files: Icon = lnir(["files-alt"])
