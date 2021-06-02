import {
    docsDescription,
    docsItem,
    docsPurpose,
    docsSection,
    docsSection_double,
} from "../../ui/vendor/getto-application/docs/helper"

import { DocsSection } from "../../ui/vendor/getto-application/docs/data"

export const docs_docs: DocsSection[] = [
    docsSection("ドキュメント", [
        docsPurpose(["業務の目標を達成する"]),
        docsItem("重要な点の明文化", [
            "重要な点が判別できる",
            "重要でない点が判別できる",
            "すべての関係者が読める",
            "書きやすい",
        ]),
    ]),
]

export const docs_privacyPolicy: DocsSection[] = [
    docsSection_double("取り扱いデータ", [
        docsDescription([
            {
                title: "ログインID / パスワード",
                body: [
                    "システムを使用するための認証に使用します",
                    "それ以外の用途で使用することはありません",
                    "パスワードは暗号化して送信、保存されます",
                ],
                help: [],
            },
        ]),
        docsDescription([
            {
                title: "メールアドレス",
                body: [
                    "パスワードリセットのために使用します",
                    "それ以外の用途で使用することはありません",
                ],
                help: [],
            },
        ]),
        docsDescription([
            {
                title: "業務データ",
                body: [
                    "システムで扱うすべてのデータは、業務を行うために使用します",
                    "業務に関係ない対象に情報が開示されることはありません",
                ],
                help: [],
            },
        ]),
    ]),
]
