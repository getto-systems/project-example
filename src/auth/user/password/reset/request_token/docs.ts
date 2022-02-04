import { DocsAction } from "../../../../../z_vendor/getto-application/docs/data"
import { docs_authUser } from "../../../docs"
import { docs_loginID } from "../../../login_id/docs"
import { docs_reset } from "../docs"

export const docs_requestResetToken: DocsAction = {
    title: "リセットトークン要求",
    action: [
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
            check: ["ログインIDに紐付いたトークンの送信先が登録されている"],
        },
        {
            type: "success",
            action: ["パスワードリセットトークンを送信"],
            help: ["リセットトークンは送信先のメールアドレスに送信される"],
        },
        {
            type: "error",
            err: ["ログインIDが無効", "ログインIDに紐付いたトークンの送信先が登録されていない"],
        },
    ],
    data: [docs_authUser, docs_reset, docs_loginID],
}
