import { DocsDescription } from "../../../z_vendor/getto-application/docs/data"
import { docsData } from "../../../z_vendor/getto-application/docs/helper"

export const docs_auth_user_loginId: DocsDescription = {
    title: "ログインID",
    descriptions: [
        {
            title: "登録",
            description: ["ユーザー登録時に指定する", "システム内で一意"],
        },
        {
            title: "管理者による変更",
            description: ["管理者はログインIDを変更できる"],
        },
    ],
}

export const docs_loginId = docsData("ログインIDデータ", [
    {
        data: "ログインID",
        help: [
            "ログインに必要なID",
            "認証ユーザーに紐づく",
            "ユーザーを一意に特定できる",
            "一定文字数を超えない",
            "ユーザーによって変更される",
        ],
    },
])
