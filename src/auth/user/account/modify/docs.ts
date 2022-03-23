import { DocsAction } from "../../../../z_vendor/getto-application/docs/data"

export const docs_overrideLoginId: DocsAction = {
    title: "ログインID上書き",
    action: [
        {
            type: "check",
            check: ["管理者権限を持っている"],
            help: ["管理者権限でログインIDを上書きする"],
        },
        {
            type: "input",
            content: ["ログインID"],
        },
        {
            type: "check",
            check: ["ログインIDが有効"],
            help: ["空でない", "一定の長さを超えない"],
        },
        {
            type: "check",
            check: ["ログインIDが登録されていない"],
        },
        {
            type: "success",
            action: ["ログインID変更完了の通知"],
        },
        {
            type: "error",
            err: ["ログインIDが無効", "ログインIDが登録されている"],
        },
    ],
    data: [],
}
