import { DocsAction } from "../../../z_vendor/getto-application/docs/data"

export const docs_notifyUnexpectedError: DocsAction = {
    title: "エラーの通知",
    action: [
        {
            type: "input",
            content: ["エラーメッセージ"],
            help: ["発生したエラーの詳細"],
        },
        {
            type: "check",
            check: ["認証済みであること"],
        },
        {
            type: "success",
            action: ["発生したエラーを収集", "エラーを通知"],
        },
    ],
    data: [],
}
