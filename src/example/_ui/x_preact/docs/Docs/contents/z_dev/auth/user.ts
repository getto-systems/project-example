import { VNode } from "preact"
import { html } from "htm/preact"

import { box, container } from "../../../../../../../../../ui/vendor/getto-css/preact/design/box"
import { v_medium } from "../../../../../../../../../ui/vendor/getto-css/preact/design/alignment"

import { itemsSection } from "../../../box"

export const content_development_auth_user = (): VNode[] => [
    container([
        box({
            title: "業務で必要な時に使用するために",
            body: [
                itemsSection("ユーザー情報を管理できる", [
                    "ログインID 変更",
                    "パスワード変更",
                    "web 証明書変更",
                ]),
            ],
        }),
        box({
            title: "業務内容をプライベートに保つために",
            body: [
                itemsSection("ユーザーの状態を管理できる", [
                    "ログイン可否の変更",
                    "アクセスを許可するコンテンツ",
                ]),
            ],
        }),
    ]),
    v_medium(),
    container(manageUser()),
]

const manageUser = () => [
    box({
        title: "ユーザー登録(あとで)",
        body: html`
            <p>チケットの検証</p>
            <p>管理者権限を確認</p>
            <p>ユーザーの登録</p>
        `,
    }),
    box({
        title: "特定ユーザーログインID 変更(あとで)管理者",
        body: html`
            <p>チケットの検証</p>
            <p>管理者権限を確認</p>
            <p>特定ユーザーのログインIDを変更</p>
        `,
    }),
    box({
        title: "特定ユーザーパスワード変更(あとで)管理者",
        body: html`
            <p>チケットの検証</p>
            <p>管理者権限を確認</p>
            <p>特定ユーザーのパスワードを変更</p>
        `,
    }),
    box({
        title: "特定ユーザーパスワード削除(あとで)管理者",
        body: html`
            <p>チケットの検証</p>
            <p>管理者権限を確認</p>
            <p>特定ユーザーのパスワードを削除</p>
        `,
    }),
    box({
        title: "特定ユーザー無効化(あとで)管理者",
        body: html`
            <p>チケットの検証</p>
            <p>管理者権限を確認</p>
            <p>特定ユーザーの全チケットの最大延長期間を削除</p>
        `,
    }),
]
