import { docsData } from "../../../z_vendor/getto-application/docs/helper"

export const docs_password = docsData("パスワードデータ", [
    {
        data: "パスワード",
        help: [
            "保存されたパスワードとハッシュが一致すれば認証成功",
            "認証ユーザーに紐づく",
            "ハッシュ化して保存",
            "一定文字数を超えない",
        ],
    },
    {
        data: "リセットトークン",
        help: ["パスワードをリセットするためのトークン", "メールで送信される"],
    },
])
