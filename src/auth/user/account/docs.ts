import { DocsDescription } from "../../../z_vendor/getto-application/docs/data"

export const docs_auth_user_account: DocsDescription = {
    title: "ユーザー管理",
    descriptions: [
        {
            title: "登録",
            description: ["ログインID、権限、パスワード、リセットトークン送信先を指定して登録する"],
        },
        {
            title: "権限",
            description: ["システムを操作するための権限", "権限に対応した操作しかできない"],
        },
        {
            title: "検索/変更",
            description: ["システム内のユーザーを検索、変更できる"],
        },
        {
            title: "削除",
            description: ["ユーザーを削除する", "削除されたユーザーはその後許可証検証が通らない"],
        },
    ],
}
