import {
    docsAction_legacy,
    docsData,
    docsDescription,
    docsModule,
    docsNote,
    docsSection,
} from "../../../../ui/vendor/getto-application/docs/helper"

import { DocsSection } from "../../../../ui/vendor/getto-application/docs/data"

export const docs_reset = docsData("パスワードリセットデータ", ({ data }) => [
    data("リセットトークン", [
        "パスワードをリセットするためのトークン",
        "ログインIDに紐付けて保存",
        "メールで送信される",
    ]),
])

export const docs_auth_resetPassword: DocsSection[] = [
    docsSection("パスワードリセット", [
        docsModule([
            "ログインID入力",
            "ログインIDでリセットトークンを発行",
            "リセットトークン送信状況のチェック",
            "ログインID・パスワード入力",
            "リセットトークンでリセット",
        ]),
    ]),
]

export const docs_auth_resetPassword_data: DocsSection[] = [
    docsSection("パスワードリセット", [
        docsDescription([
            {
                title: "リセットセッションID",
                body: ["セッションを特定するための文字列"],
                help: [
                    "リセットセッションを一意に特定",
                    "このセッションIDでトークンの送信状況を確認",
                    "リセットセッションが失効・完了するまで有効",
                ],
            },
        ]),
        docsDescription([
            {
                title: "リセットトークン",
                body: ["セッションを特定するための文字列"],
                help: [
                    "リセットセッションを一意に特定",
                    "テキストメッセージで配信",
                    "このトークンでパスワードをリセット",
                    "リセットセッションが失効・完了するまで有効",
                ],
            },
        ]),
    ]),
]

export const docs_auth_resetPassword_description: DocsSection[] = [
    ...docs_auth_resetPassword,

    docsSection("ログインID入力", [
        docsAction_legacy(({ action, validate }) => [
            action({
                on: "http-client",
                body: [...validate(["ログインID"])],
                help: ["空でないこと", "一定の長さ以下であること"],
            }),
        ]),
        docsNote(["検証失敗の場合はリクエストしない"]),
    ]),
    docsSection("ログインIDでリセットトークンを発行", [
        docsAction_legacy(({ request, action, validate, message }) => [
            request({
                from: "http-client",
                to: "api-server",
                body: [...message(["ログインID"])],
                help: [],
            }),
            action({
                on: "api-server",
                body: [...validate(["ログインID"]), ...message(["リセットトークン発行"])],
                help: ["ログインIDに宛先が紐づいているなら発行"],
            }),
            request({
                from: "api-server",
                to: "text-client",
                body: [...message(["リセットセッションID"])],
                help: ["テキストメッセージにリセットセッションIDを埋め込んで送信"],
            }),
        ]),
        docsNote(["ログインIDに宛先が紐づいていない場合、リセットトークンは発行できない"]),
    ]),
    docsSection("リセットトークン送信状況のチェック", [
        docsAction_legacy(({ request, action, validate, message }) => [
            request({
                from: "http-client",
                to: "api-server",
                body: [...message(["リセットセッションID"])],
                help: [],
            }),
            action({
                on: "api-server",
                body: [
                    ...validate(["リセットセッションID"]),
                    ...message(["リセットトークン配信状況取得"]),
                ],
                help: ["リセットセッションIDがアクティブなら情報を取得可能"],
            }),
            request({
                from: "api-server",
                to: "text-client",
                body: [...message(["リセットトークン配信状況"])],
                help: [],
            }),
        ]),
        docsNote(["セッションIDが存在しないならエラー", "セッション完了なら完了済みエラー"]),
    ]),
    docsSection("ログインID・パスワード入力", [
        docsAction_legacy(({ request, validate }) => [
            request({
                from: "http-client",
                to: "api-server",
                body: [...validate(["ログインID・パスワード"])],
                help: ["空でないこと", "一定の長さ以下であること"],
            }),
        ]),
        docsNote(["検証失敗の場合はリクエストしない"]),
    ]),
    docsSection("リセットトークンでリセット", [
        docsAction_legacy(({ request, action, validate, message }) => [
            request({
                from: "http-client",
                to: "api-server",
                body: [...message(["ログインID・パスワード", "リセットトークン"])],
                help: [],
            }),
            action({
                on: "api-server",
                body: [
                    ...validate(["ログインID・パスワード"]),
                    ...validate(["リセットトークン"]),
                    ...message(["パスワードリセット"]),
                    ...message(["認証トークン発行", "認可トークン発行", "コンテンツトークン発行"]),
                ],
                help: [
                    "ログインIDはセッション開始時に入力したものと一致することを確認",
                    "リセットトークンが存在することを確認",
                ],
            }),
            request({
                from: "api-server",
                to: "http-client",
                body: [...message(["認証チケット"])],
                help: ["各トークンは cookie へ登録"],
            }),
        ]),
        docsNote([
            "リセットトークンに紐づいたセッションがが存在しないならエラー",
            "セッション完了なら完了済みエラー",
        ]),
    ]),
]
