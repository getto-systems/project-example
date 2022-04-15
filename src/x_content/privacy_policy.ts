import { DocsDescription } from "../z_vendor/getto-application/docs/data"

export const content_privacyPolicy: DocsDescription = {
    title: "取り扱いデータ",
    descriptions: [
        {
            title: "ログインID / パスワード",
            description: [
                "システムを使用するための認証に使用します",
                "それ以外の用途で使用することはありません",
                "パスワードは暗号化して送信、保存されます",
            ],
        },
        {
            title: "メールアドレス",
            description: [
                "パスワードリセットのために使用します",
                "それ以外の用途で使用することはありません",
            ],
        },
        {
            title: "業務データ",
            description: [
                "システムで扱うすべてのデータは、業務を行うために使用します",
                "業務に関係ない対象に情報が開示されることはありません",
            ],
        },
    ],
}
