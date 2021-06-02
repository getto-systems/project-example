import { docsModule, docsPurpose, docsSection } from "../../ui/vendor/getto-application/docs/helper"

import { DocsSection } from "../../ui/vendor/getto-application/docs/data"

export const docs_example: DocsSection[] = [
    docsSection("GETTO Example", [
        docsModule(["業務アプリケーションのひな型", "このコードをコピーして始める"]),
    ]),
    docsSection("業務アプリケーション", [
        docsPurpose([
            "業務の目標を達成する",
            "業務で必要な時に使用できる",
            "業務に合ったコストで運用できる",
            "業務内容をプライベートに保つ",
        ]),
    ]),
]
