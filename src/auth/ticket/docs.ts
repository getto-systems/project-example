import { docsData } from "../../../ui/vendor/getto-application/docs/helper"

export const docs_authTicket = docsData("認証チケットデータ", [
    {
        data: "認証チケット",
        help: [
            "認証時に発行される許可証",
            "認証ユーザーに紐づく",
            "認証成功ごとに新しく発行される",
            "有効期限延長の最大期限をもつ",
        ],
    },
    {
        data: "認証チケット延長トークン",
        help: [
            "有効期限延長のためのトークン",
            "認証チケットを元に生成される",
            "セキュアな Cookie として返される",
            "適切に署名される",
            "有効期限をもつ",
        ],
    },
    {
        data: "API アクセストークン",
        help: [
            "API にアクセスするためのトークン",
            "認証チケットを元に生成される",
            "セキュアな Cookie として返される",
            "適切に署名される",
            "有効期限をもつ",
        ],
    },
    {
        data: "コンテンツアクセストークン",
        help: [
            "アプリケーションを読み込むためのトークン",
            "認証チケットを元に生成される",
            "Http Only な Cookie として返される",
            "適切に署名される",
            "有効期限をもつ",
        ],
    },
])
