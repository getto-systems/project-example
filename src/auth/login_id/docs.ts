import { docsDescription, docsSection } from "../../../ui/vendor/getto-application/docs/helper"

import { DocsSection } from "../../../ui/vendor/getto-application/docs/data"

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
