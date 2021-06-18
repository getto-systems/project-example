import { docsAction } from "../../../../../../ui/vendor/getto-application/docs/helper"

export const docs_requestResetToken = docsAction("リセットトークン要求", ({ item }) => [
    item("input", ["ログインID"]),
    item("check", ["ログインIDが有効"], ["空でない", "一定の長さを超えない"]),
    item("check", ["ログインIDに紐付いたトークンの送信先が登録されている"]),
    item(
        "success",
        ["パスワードリセットトークンを送信"],
        ["リセットトークンは送信先のメールアドレスに送信される"],
    ),
    item("error", ["ログインIDが無効", "ログインIDに紐付いたトークンの送信先が登録されていない"]),
])
