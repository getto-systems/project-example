import { DocsAction } from "../../../../ui/vendor/getto-application/docs/data"

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
            action: ["エラーを通知"],
        },
    ],
    data: [],
}
