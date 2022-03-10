import { DocsAction } from "../../../../z_vendor/getto-application/docs/data"

export const docs_changePassword: DocsAction = {
    title: "パスワード変更",
    action: [
        {
            type: "input",
            content: ["現在のパスワード", "新しいパスワード"],
        },
        {
            type: "check",
            check: ["現在のパスワードが有効", "新しいパスワードが有効"],
            help: ["空でない", "一定の長さを超えない"],
        },
        {
            type: "check",
            check: ["現在のパスワードが登録されたものと一致する"],
        },
        {
            type: "success",
            action: ["パスワード変更完了の通知"],
        },
        {
            type: "error",
            err: ["パスワードが無効", "パスワードが登録されたものと一致しない"],
        },
    ],
    data: [],
}

export const docs_overridePassword: DocsAction = {
    title: "パスワード上書き",
    action: [
        {
            type: "check",
            check: ["管理者権限を持っている"],
            help: ["管理者権限でパスワードを上書きする"],
        },
        {
            type: "input",
            content: ["パスワード"],
        },
        {
            type: "check",
            check: ["パスワードが有効"],
            help: ["空でない", "一定の長さを超えない"],
        },
        {
            type: "success",
            action: ["パスワード変更完了の通知"],
        },
        {
            type: "error",
            err: ["パスワードが無効"],
        },
    ],
    data: [],
}
