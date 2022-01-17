import { docsData } from "../../../ui/vendor/getto-application/docs/helper"

export const docs_authUser = docsData("認証ユーザーデータ", [
    {
        data: "認証ユーザー",
        help: ["アプリケーションを使用するユーザー"],
    },
    {
        data: "認証ユーザーID",
        help: [
            "認証ユーザーを一意に特定するID",
            "一定文字数を超えない",
            "登録時に自動生成される",
            "登録後は変更されない",
        ],
    },
    {
        data: "権限",
        help: ["アプリケーションを使用するために権限が付与される"],
    },
])
