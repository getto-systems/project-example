import { docsAction } from "../../../../../ui/vendor/getto-application/docs/helper"

export const docs_notifyUnexpectedError = docsAction("認証チケットの確認", ({ item }) => [
    item("input", ["エラーメッセージ"], ["発生したエラーの詳細"]),
    item("check", ["認証済みであること"]),
    item("success", ["エラーの通知"]),
])
