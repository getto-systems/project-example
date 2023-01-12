import { lnir } from "../../common/util/icon/init/line_icon"

import { icon_home } from "../icon"

import { category, item } from "./common"
import { authPermissionLabel } from "../permission"

import { MenuContent, MenuPermissionRequired } from "../../common/outline/load/infra"

export function homeMenuContent(): MenuContent {
    return {
        key: "home",
        loadMenuBadge: true,
        menuTree: [
            category("MAIN", nothing, [
                item("ホーム", icon_home, "index.html"),
                item("ドキュメント", lnir(["files-alt"]), "docs/index.html"),
            ]),
            category("シーズン", nothing, [item("設定", lnir(["cog"]), "season/setup.html")]),
            category(menuLabel(authUser), authUser, [
                item("一覧", lnir(["friends"]), "auth/user/account.html"),
                item("登録", lnir(["add-user"]), "auth/user/register.html"),
            ]),
            category("SYSTEM", nothing, [
                item("プロフィール", lnir(["user"]), "auth/profile.html"),
                item("ログアウト", lnir(["exit"]), "auth/ticket/logout.html"),
            ]),
        ],
    }
}

const nothing: MenuPermissionRequired = { type: "nothing" } as const
const authUser: MenuPermissionRequired = { type: "has-some", permissions: ["auth-user"] } as const

function menuLabel(required: MenuPermissionRequired): string {
    switch (required.type) {
        case "nothing":
            return ""

        case "has-some":
            if (required.permissions.length === 0) {
                return ""
            }
            return authPermissionLabel(required.permissions[0])
    }
}
