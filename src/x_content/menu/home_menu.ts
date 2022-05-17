import { lnir } from "../../z_lib/ui/icon/init/line_icon"

import { icon_home } from "../icon"

import { category, item } from "./common"

import { MenuContent } from "../../common/outline/load/infra"
import { authRoleLabel } from "../role"

export function homeMenuContent(): MenuContent {
    return {
        key: "home",
        loadMenuBadge: true,
        menuTree: [
            category("MAIN", allow, [
                item("ホーム", icon_home, "index.html"),
                item("ドキュメント", lnir(["files-alt"]), "docs/index.html"),
            ]),
            category("シーズン", allow, [item("設定", lnir(["cog"]), "season/setup.html")]),
            category(authRoleLabel(authUser.role), authUser, [
                item("一覧", lnir(["friends"]), "auth/user/account.html"),
                item("登録", lnir(["add-user"]), "auth/user/register.html"),
            ]),
            category("SYSTEM", allow, [
                item("プロフィール", lnir(["user"]), "auth/profile.html"),
                item("ログアウト", lnir(["exit"]), "auth/ticket/logout.html"),
            ]),
        ],
    }
}

const allow = { type: "allow" } as const
const authUser = { type: "role", role: "auth-user" } as const
