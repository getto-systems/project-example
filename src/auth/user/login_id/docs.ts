import {
    docsData,
    docsDescription,
    docsSection,
} from "../../../../ui/vendor/getto-application/docs/helper"

import { DocsSection } from "../../../../ui/vendor/getto-application/docs/data"

export const docs_loginID = docsData("ログインIDデータ", [
    {
        data: "ログインID",
        help: [
            "ログインに必要なID",
            "認証ユーザーに紐づく",
            "ユーザーを一意に特定できる",
            "一定文字数を超えない",
            "ユーザーによって変更される",
        ],
    },
])

export const docs_auth_loginID: DocsSection[] = [
    docsSection("ログインID", [
        docsDescription([
            {
                title: "ログインID",
                body: ["ログインに必要なID"],
                help: ["ユーザーを一意に特定できる", "一定文字数を超えない"],
            },
        ]),
    ]),
]
