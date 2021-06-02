import { DocsSection } from "../../../../ui/vendor/getto-application/docs/data"
import { docsDescription, docsSection } from "../../../../ui/vendor/getto-application/docs/helper"

export const docs_auth_password: DocsSection[] = [
    docsSection("パスワード", [
        docsDescription([
            {
                title: "パスワード",
                body: ["認証のための文字列"],
                help: [
                    "暗号化して保存",
                    "保存されたパスワードと一致で認証成功",
                    "72バイトを超えない",
                ],
            },
        ]),
    ]),
]
