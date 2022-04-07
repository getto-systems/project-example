import { env } from "../../y_environment/ui/env"

import { lnir } from "../../z_lib/ui/icon/init/line_icon"
import { icon_home } from "../icon"

import { assertMenuPath, category, item } from "./common"

import { docs_auth } from "../../auth/docs"
import { docs_avail } from "../../avail/docs"
import { docs_docs } from "../../docs/docs"

import { MenuContent, MenuPermission, MenuTreeNode } from "../../core/outline/load/infra"

import { DocsDomain } from "../../z_vendor/getto-application/docs/data"
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
            docsMenuNode(docs_auth),
            item(docs_avail.title, files, "docs/avail.html"),
            item(docs_docs.title, files, "docs/docs.html"),
            item("coverage api", files, "coverage/api/index.html"),
            item("coverage ui", files, "coverage/ui/lcov-report/index.html"),
        ]),
    ]
}

function docsMenuNode(docs: DocsDomain): MenuTreeNode {
    return category(docs.title, allow, [
        item("概要", files, assertMenuPath(`docs/${docs.path}/index.html`)),
        ...docs.usecase.map((usecase) => {
            return item(
                usecase.title,
                files,
                assertMenuPath(`docs/${docs.path}/${usecase.path}.html`),
            )
        }),
    ])
}

const allow: MenuPermission = { type: "allow" }

const files: Icon = lnir(["files-alt"])
