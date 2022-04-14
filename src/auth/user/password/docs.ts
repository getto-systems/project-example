import { DocsDescription } from "../../../z_vendor/getto-application/docs/data"
import { docsData } from "../../../z_vendor/getto-application/docs/helper"

export const docs_auth_user_password: DocsDescription = {
    title: "パスワード認証",
    descriptions: [
        {
            title: "認証",
            description: [
                "ログインIDとパスワードで認証する",
                "登録された暗号化パスワードと一致するか検証する",
            ],
        },
        {
            title: "パスワードリセット",
            description: [
                "リセットトークン送信先にリセットトークンを送信する",
                "リセットトークンが検証されたら新しいパスワードに変更する",
            ],
        },
        {
            title: "リセットトークン送信先",
            description: [
                "ログインIDに紐づいた送信先",
                "リセットトークンが送信される",
                "送信は email を想定",
            ],
        },
        {
            title: "変更",
            description: ["現在のパスワードが検証されたら新しいパスワードに変更する"],
        },
        {
            title: "管理者による変更",
            description: ["管理者は現在のパスワードの検証なしにパスワードを変更できる"],
        },
    ],
}

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
