import { lnir } from "../../z_lib/ui/icon/line_icon"
import { category, item } from "./common"

import { MenuContent, MenuPermission } from "../../example/outline/load/infra"

export function homeMenuContent(): MenuContent {
    return {
        key: "home",
        loadMenuBadge: true,
        menuTree: [
            category("MAIN", allow, [
                item("ホーム", lnir("home"), "index.html"),
                item("ドキュメント", lnir("files-alt"), "docs/index.html"),
            ]),
            category("ACCOUNT", user, [item("ユーザー", lnir("user"), "auth/user/account.html")]),
            category("シーズン", allow, [item("設定", lnir("cog"), "example/season/setup.html")]),
            category("SYSTEM", allow, [
                item("プロフィール", lnir("user"), "auth/profile.html"),
                item("ログアウト", lnir("user"), "auth/ticket/logout.html"),
            ]),
        ],
    }
}

const allow: MenuPermission = { type: "allow" }
const user: MenuPermission = { type: "allow" } // TODO { type: "role", role: "manage_auth_user" } ロールの編集ができるようになったら制限する
