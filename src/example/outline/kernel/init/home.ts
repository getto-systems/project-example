import { lnir } from "../../../../z_lib/ui/icon/line_icon"
import { MenuContent, MenuPermission } from "../infra"
import { category, item } from "./common"

// TODO このファイル名とかファイルのロケーションとか考え直したい

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
            category("SYSTEM", allow, [
                item("プロフィール", lnir("user"), "auth/profile.html"),
                item("ログアウト", lnir("user"), "auth/ticket/logout.html"),
            ]),
        ],
    }
}

const allow: MenuPermission = { type: "allow" }
const user: MenuPermission = { type: "allow" } // TODO { type: "role", role: "user" } ロールの編集ができるようになったら制限する
