import { DocsAction } from "../../../z_vendor/getto-application/docs/data"

import { docs_authTicket } from "../docs"

export const docs_checkAuthTicket: DocsAction = {
    title: "認証チケットの確認",
    action: [
        {
            type: "input",
            content: ["認証チケット有効期限", "認証チケット延長トークン"],
            help: ["ブラウザに保存されたデータ"],
        },
        {
            type: "check",
            check: ["認証チケットが有効", "認証チケット延長トークンが有効"],
        },
        {
            type: "success",
            action: ["アプリケーションのロード", "認証チケット継続更新の開始"],
            help: ["コンテンツアクセストークンが cookie で返される"],
        },
        {
            type: "error",
            err: ["認証チケット有効期限切れ", "認証チケット延長トークン無効"],
            help: ["ログイン画面へ"],
        },
    ],
    data: [docs_authTicket],
}
