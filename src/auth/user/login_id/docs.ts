import { docsData } from "../../../z_vendor/getto-application/docs/helper"

export const docs_loginID = docsData("ログインIDデータ", [
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
