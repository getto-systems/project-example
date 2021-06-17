import {
    docsAction,
    docsAction_legacy,
    docsModule,
    docsNote,
    docsSection,
} from "../../../../../ui/vendor/getto-application/docs/helper"

import { DocsSection } from "../../../../../ui/vendor/getto-application/docs/data"

export const docs_auth_logout: DocsSection[] = [
    docsSection("ログアウト", [docsModule(["認証チケットの無効化"])]),
]

export const docs_logout = docsAction("ログアウト", ({ item }) => [
    item(
        "input",
        ["認証チケット延長トークン"],
        ["ブラウザに保存されたデータ"],
    ),
    item("check", ["認証チケットが有効", "認証チケット延長トークンが有効"]),
    item(
        "success",
        ["ログアウト", "認証チケットの破棄"],
        ["ログイン画面へ"],
    ),
    item("error", ["認証チケット有効期限切れ"], ["ログイン画面へ"]),
    item("error", ["認証チケット延長トークン無効"], ["ログイン画面へ"]),
])

export const docs_auth_logout_description: DocsSection[] = [
    ...docs_auth_logout,

    docsSection("認証チケットの無効化", [
        docsAction_legacy(({ request, action, validate, message }) => [
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
