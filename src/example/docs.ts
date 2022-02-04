import { DocsAction, DocsDomain } from "../z_vendor/getto-application/docs/data"

const docs_app: DocsAction = {
    title: "業務アプリケーション",
    action: [
        {
            type: "check",
            check: [
                "業務の目標を達成する",
                "業務で必要な時に使用できる",
                "業務に合ったコストで運用できる",
                "業務内容をプライベートに保つ",
            ],
        },
    ],
    data: [],
}

export const docs_example: DocsDomain = {
    path: "docs",
    title: "ドキュメント",
    usecase: [
        {
            path: "app",
            title: docs_app.title,
            purpose: ["業務アプリケーションのひな型", "このコードをコピーして始める"],
            action: [docs_app],
        },
    ],
}
