import { DocsAction } from "../../../../../z_vendor/getto-application/docs/data"
import { docs_authUser } from "../../../docs"
import { docs_loginID } from "../../../login_id/docs"
import { docs_password } from "../../docs"
import { docs_reset } from "../docs"

export const docs_resetPassword: DocsAction = {
    title: "パスワードリセット",
    action: [
        {
            type: "input",
            content: ["リセットトークン", "ログインID", "パスワード"],
            help: ["リセットトークンはメールで送信される"],
        },
        {
            type: "check",
            check: ["リセットトークンが有効"],
            help: ["空でない", "登録されている", "有効期限が切れていない"],
        },
        {
            type: "check",
            check: ["ログインIDが有効"],
            help: [
                "空でない",
                "一定の長さを超えない",
                "リセットしようとしているログインIDと一致する",
            ],
        },
        {
            type: "check",
            check: ["パスワードが有効"],
            help: ["空でない", "一定の長さを超えない"],
        },
        {
            type: "success",
            action: ["パスワードの変更", "アプリケーションのロード", "認証チケット継続更新の開始"],
            help: ["コンテンツアクセストークンが cookie で返される"],
        },
        {
            type: "error",
            err: ["リセットトークンかログインIDかパスワードが無効"],
        },
    ],
    data: [docs_authUser, docs_reset, docs_loginID, docs_password],
}
