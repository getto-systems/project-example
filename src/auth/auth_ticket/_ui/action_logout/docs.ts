import {
    docsAction,
    docsModule,
    docsNote,
    docsSection,
} from "../../../../../ui/vendor/getto-application/docs/helper"

import { DocsSection } from "../../../../../ui/vendor/getto-application/docs/data"

export const docs_auth_logout: DocsSection[] = [
    docsSection("ログアウト", [docsModule(["認証チケットの無効化"])]),
]

export const docs_auth_logout_description: DocsSection[] = [
    ...docs_auth_logout,

    docsSection("認証チケットの無効化", [
        docsAction(({ request, action, validate, message }) => [
            request({
                from: "http-client",
                to: "api-server",
                body: [...message(["認証トークン・nonce"])],
                help: [],
            }),
            action({
                on: "api-server",
                body: [
                    ...validate(["認証トークン・nonce 検証", "認証チケット有効期限検証"]),
                    ...message(["認証チケット無効化"]),
                ],
                help: [],
            }),
            action({
                on: "http-client",
                body: [...message(["認証チケット情報の破棄"])],
                help: [],
            }),
        ]),
        docsNote(["処理完了でログイン画面に遷移"]),
    ]),
]
