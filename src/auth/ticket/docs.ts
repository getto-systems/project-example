import { DocsDescription } from "../../z_vendor/getto-application/docs/data"
import { docsData } from "../../z_vendor/getto-application/docs/helper"

export const docs_auth_ticket: DocsDescription = {
    title: "許可証",
    descriptions: [
        {
            title: "検証",
            description: [
                "以下の検証をパスしないとアクセスが許可されない",
                "・権限",
                "・有効期限",
                "検証完了で有効期限を延長した許可証を発行する",
            ],
        },
        {
            title: "発行",
            description: [
                "認証完了で発行する",
                "システム外部で保管される",
                "保管はブラウザの cookie を想定",
            ],
        },
        {
            title: "ログアウト",
            description: ["許可証を無効化する", "無効化した許可証は検証を通らない"],
        },
    ],
}

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
