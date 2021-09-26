import { docsAction } from "../../../../../ui/vendor/getto-application/docs/helper"

export const docs_resetPassword = docsAction("パスワードリセット", ({ item }) => [
    item(
        "input",
        ["リセットトークン", "ログインID", "パスワード"],
        ["リセットトークンはメールで送信される"],
    ),
    item(
        "check",
        ["リセットトークンが有効"],
        ["空でない", "登録されている", "有効期限が切れていない"],
    ),
    item(
        "check",
        ["ログインIDが有効"],
        ["空でない", "一定の長さを超えない", "リセットしようとしているログインIDと一致する"],
    ),
    item("check", ["パスワードが有効"], ["空でない", "一定の長さを超えない"]),
    item(
        "success",
        ["パスワードの変更", "アプリケーションのロード", "認証チケット継続更新の開始"],
        ["コンテンツアクセストークンが cookie で返される"],
    ),
    item("error", ["リセットトークンかログインIDかパスワードが無効"]),
])
