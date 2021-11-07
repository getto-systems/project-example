import { env } from "../../../../y_environment/ui/env"

import { lnir } from "../../../../z_lib/ui/icon/line_icon"

import { assertMenuPath, category, item } from "./common"

import { docs_auth } from "../../../../auth/docs"
import { docs_avail } from "../../../../avail/docs"

import { MenuContent, MenuPermission, MenuTreeNode } from "../infra"

import { DocsDomain } from "../../../../../ui/vendor/getto-application/docs/data"

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
                docsMenuNode(docs_auth, "auth"),
                item(docs_avail.title, lnir("files-alt"), "docs/avail.html"),
            ]),
            ...devDocs(),
        ],
    }
}
function devDocs(): MenuTreeNode[] {
    if (env.isProduction) {
        return []
    }
    return [
        category("開発用", dev, [
            item("Storybook", lnir("files-alt"), "storybook/index.html"),
            item("coverage api", lnir("files-alt"), "coverage/api/index.html"),
            item("coverage ui", lnir("files-alt"), "coverage/ui/lcov-report/index.html"),
        ]),
    ]
}

function docsMenuNode<U, A, D>(domain: DocsDomain<U, A, D>, path: string): MenuTreeNode {
    return category(domain.title, allow, [
        item("概要", lnir("files-alt"), assertMenuPath(`docs/${path}/index.html`)),
        ...domain.usecase.map((name) => {
            const usecase = domain.toUsecase(name)
            const action = usecase.toAction(usecase.title)
            return item(
                action.title,
                lnir("files-alt"),
                assertMenuPath(`docs/${path}/${`${usecase.path}`}.html`),
            )
        }),
    ])
}

const allow: MenuPermission = { type: "allow" }
const dev: MenuPermission = { type: "role", role: "dev-docs" }
