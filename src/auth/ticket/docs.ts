import { DocsDescription } from "../../z_vendor/getto-application/docs/data"

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
