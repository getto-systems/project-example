import { env } from "../../../../y_environment/_ui/env"

import { lnir } from "../../../../z_details/_ui/icon/line_icon"

import { category, item } from "./common"

import { MenuContent, MenuPermission } from "../infra"

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
                item("認証・認可", lnir("files-alt"), "/docs/auth.html"),
                category("認証・認可", dev, [
                    item("認証", lnir("files-alt"), "/docs/z-dev/auth/sign.html"),
                ]),
                item("保守・運用", lnir("files-alt"), "/docs/avail.html"),
            ]),
            ...(env.isProduction
                ? []
                : [
                      category("開発用", dev, [
                          item("Storybook", lnir("files-alt"), "/storybook/index.html"),
                          item("coverage", lnir("files-alt"), "/coverage/lcov-report/index.html"),
                      ]),
                  ]),
        ],
    }
}

const allow: MenuPermission = { type: "allow" }
const dev: MenuPermission = { type: "role", role: "dev-docs" }
