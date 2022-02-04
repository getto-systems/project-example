import { env } from "../../y_environment/ui/env"

import { lnir } from "../../z_lib/ui/icon/line_icon"
import { assertMenuPath, category, item } from "./common"

import { docs_auth } from "../../auth/docs"
import { docs_avail } from "../../avail/docs"

import { MenuContent, MenuPermission, MenuTreeNode } from "../../core/outline/load/infra"

import { DocsDomain } from "../../z_vendor/getto-application/docs/data"
import { docs_docs } from "../../docs/docs"

export function docsMenuContent(): MenuContent {
    return {
        key: "docs",
        loadMenuBadge: false,
        menuTree: [
            category("MAIN", allow, [
                item("ホーム", lnir("home"), "index.html"),
                item("ドキュメント", lnir("files-alt"), "docs/index.html"),
                item("プライバシーポリシー", lnir("files-alt"), "docs/privacy-policy.html"),
            ]),
            category("ドキュメント", allow, [
                docsMenuNode(docs_auth),
                item(docs_avail.title, lnir("files-alt"), "docs/avail.html"),
                item(docs_docs.title, lnir("files-alt"), "docs/docs.html"),
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
        category("開発用", dev, [
            item("coverage api", lnir("files-alt"), "coverage/api/index.html"),
            item("coverage ui", lnir("files-alt"), "coverage/ui/lcov-report/index.html"),
        ]),
    ]
}

function docsMenuNode(docs: DocsDomain): MenuTreeNode {
    return category(docs.title, allow, [
        item("概要", lnir("files-alt"), assertMenuPath(`docs/${docs.path}/index.html`)),
        ...docs.usecase.map((usecase) => {
            return item(
                usecase.title,
                lnir("files-alt"),
                assertMenuPath(`docs/${docs.path}/${usecase.path}.html`),
            )
        }),
    ])
}

const allow: MenuPermission = { type: "allow" }
const dev: MenuPermission = { type: "role", role: "dev-docs" }
