import { DocsAction } from "../../../z_vendor/getto-application/docs/data"
import { docs_authUser } from "../../user/docs"
import { docs_authTicket } from "../docs"

export const docs_logout: DocsAction = {
    title: "ログアウト",
    action: [
        {
            type: "input",
            content: ["認証チケット延長トークン"],
            help: ["ブラウザに保存されたデータ"],
        },
        {
            type: "check",
            check: ["認証チケットが有効", "認証チケット延長トークンが有効"],
        },
        {
            type: "success",
            action: ["ログアウト", "認証チケットの破棄"],
            help: ["ログイン画面へ"],
        },
        {
            type: "error",
            err: ["認証チケット有効期限切れ", "認証チケット延長トークン無効"],
            help: ["ログイン画面へ"],
        },
    ],
    data: [docs_authUser, docs_authTicket],
}
