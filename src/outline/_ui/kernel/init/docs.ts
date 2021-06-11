import { env } from "../../../../y_environment/_ui/env"

import { lnir } from "../../../../z_details/_ui/icon/line_icon"

import { assertMenuPath, category, item } from "./common"

import { docs_auth } from "../../../../auth/docs"

import { MenuContent, MenuPermission, MenuTreeNode } from "../infra"

import { DocsDomain } from "../../../../../ui/vendor/getto-application/docs/data"

export function docsMenuContent(): MenuContent {
    return {
        database: env.database.menuExpand,
        key: "docs",
        loadMenuBadge: false,
        menuTree: [
            category("MAIN", allow, [
                item("ホーム", lnir("home"), "/index.html"),
                item("ドキュメント", lnir("files-alt"), "/docs/index.html"),
                item("プライバシーポリシー", lnir("files-alt"), "/docs/privacy-policy.html"),
            ]),
            category("ドキュメント", allow, [
                docsMenuNode(docs_auth, "auth"),
                item("保守・運用", lnir("files-alt"), "/docs/avail.html"),
            ]),
            ...(env.isProduction
                ? []
                : [
                      category("開発用", dev, [
                          item("Storybook", lnir("files-alt"), "/storybook/index.html"),
                          item("coverage api", lnir("files-alt"), "/coverage/api/index.html"),
                          item("coverage ui", lnir("files-alt"), "/coverage/ui/lcov-report/index.html"),
                      ]),
                  ]),
        ],
    }
}

function docsMenuNode<U, A, D>(domain: DocsDomain<U, A, D>, path: string): MenuTreeNode {
    return category(domain.title, allow, [
        item("概要", lnir("files-alt"), assertMenuPath(`/docs/${path}/index.html`)),
        ...domain.usecase.map((name) => {
            const usecase = domain.toUsecase(name)
            const action = usecase.toAction(usecase.title)
            return item(
                action.title,
                lnir("files-alt"),
                assertMenuPath(
                    `/docs/${path}/${`${name}`.replaceAll(
                        /[A-Z]/g,
                        (char) => `-${char.toLowerCase()}`,
                    )}.html`,
                ),
            )
        }),
    ])
}

const allow: MenuPermission = { type: "allow" }
const dev: MenuPermission = { type: "role", role: "dev-docs" }
