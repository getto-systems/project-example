import {
    docsAction_legacy,
    docsModule,
    docsNote,
    docsSection,
} from "../../../../../ui/vendor/getto-application/docs/helper"

import { DocsSection } from "../../../../../ui/vendor/getto-application/docs/data"

export const docs_auth_authenticatePassword: DocsSection[] = [
    docsSection("パスワードログイン", [
        docsModule(["ログインID・パスワード入力", "ログインID・パスワード認証"]),
    ]),
]

export const docs_auth_authenticatePassword_description: DocsSection[] = [
    ...docs_auth_authenticatePassword,

    docsSection("ログインID・パスワード入力", [
        docsAction_legacy(({ action, validate }) => [
            action({
                on: "http-client",
                body: [...validate(["ログインID・パスワード"])],
                help: ["空でないこと", "一定の長さ以下であること"],
            }),
        ]),
        docsNote(["検証失敗の場合はリクエストしない"]),
    ]),
    docsSection("ログインID・パスワード認証", [
        docsAction_legacy(({ request, action, validate, message }) => [
            request({
                from: "http-client",
                to: "api-server",
                body: [...message(["ログインID・パスワード"])],
                help: [],
            }),
            action({
                on: "api-server",
                body: [
                    ...validate(["ログインID・パスワード"]),
                    ...message(["認証トークン発行", "認可トークン発行", "コンテンツトークン発行"]),
                ],
                help: ["暗号化パスワードとの一致検証"],
            }),
            request({
                from: "api-server",
                to: "http-client",
                body: [...message(["認証チケット"])],
                help: ["各トークンは cookie へ登録"],
            }),
        ]),
    ]),
]
