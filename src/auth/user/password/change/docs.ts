import { DocsAction } from "../../../../z_vendor/getto-application/docs/data"

export const docs_changePassword: DocsAction = {
    title: "パスワード変更",
    action: [
        {
            type: "input",
            content: ["ログインID", "パスワード"],
        },
        {
            type: "check",
            check: ["ログインIDが有効", "パスワードが有効"],
            help: ["空でない", "一定の長さを超えない"],
        },
        {
            type: "check",
            check: ["ログインIDが登録されている", "パスワードが登録されたものと一致する"],
        },
        {
            type: "success",
            action: ["アプリケーションのロード", "認証チケット継続更新の開始"],
            help: ["コンテンツアクセストークンが cookie で返される"],
        },
        {
            type: "error",
            err: [
                "ログインIDかパスワードが無効",
                "ログインIDが登録されていない",
                "パスワードが登録されたものと一致しない",
            ],
        },
    ],
    data: [],
}
