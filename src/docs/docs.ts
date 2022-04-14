import { DocsAction, DocsDescription, DocsDomain } from "../z_vendor/getto-application/docs/data"

export const docs_docs: DocsDescription = {
    title: "ドキュメント",
    descriptions: [
        {
            title: "重要な点の明文化",
            description: [
                "重要な点が判別できる",
                "重要でない点が判別できる",
                "すべての関係者が読める",
                "書きやすい",
            ],
        },
    ],
}

export const docs_dataHandling: DocsAction = {
    title: "取り扱いデータ",
    action: [
        {
            type: "input",
            content: ["ログインID", "パスワード"],
            help: [
                "システムを使用するための認証に使用します",
                "それ以外の用途で使用することはありません",
                "パスワードは暗号化して送信、保存されます",
            ],
        },
        {
            type: "input",
            content: ["メールアドレス"],
            help: [
                "パスワードリセットのために使用します",
                "それ以外の用途で使用することはありません",
            ],
        },
        {
            type: "input",
            content: ["業務データ"],
            help: [
                "システムで扱うすべてのデータは、業務を行うために使用します",
                "業務に関係ない対象に情報が開示されることはありません",
            ],
        },
    ],
    data: [],
}

export const docs_privacyPolicy: DocsDomain = {
    path: "docs",
    title: "プライバシーポリシー",
    usecase: [
        {
            path: "private-data",
            title: docs_dataHandling.title,
            purpose: ["業務内容をプライベートに保つ"],
            action: [docs_dataHandling],
        },
    ],
}
