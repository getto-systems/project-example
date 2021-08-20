import { docsAction } from "../../../../../ui/vendor/getto-application/docs/helper"

export const docs_notifyUnexpectedError = docsAction("エラーの通知", ({ item }) => [
    item("input", ["エラーメッセージ"], ["発生したエラーの詳細"]),
    item("check", ["認証済みであること"]),
    item("success", ["エラーを通知"]),
])
