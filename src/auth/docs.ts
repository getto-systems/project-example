import { docs_auth_sign, docs_auth_sign_action, docs_auth_sign_data } from "./sign/sign/docs"
import { docs_checkAuthTicket } from "./ticket/check/docs"

import {
    docsModule,
    docsPurpose,
    docsSection,
    docsSection_pending,
} from "../../ui/vendor/getto-application/docs/helper"

import { DocsAction, DocsDomain, DocsSection } from "../../ui/vendor/getto-application/docs/data"
import { docs_authTicket } from "./ticket/docs"
import { docs_authenticatePassword } from "./user/password/authenticate/docs"
import { docs_logout } from "./ticket/logout/docs"
import { docs_requestResetToken } from "./user/password/reset/request_token/docs"
import { docs_resetPassword } from "./user/password/reset/reset/docs"
import { docs_changePassword } from "./user/password/change/docs"

const docs_loadApplication: DocsAction = {
    title: "アプリケーションのロード",
    action: [
        {
            type: "input",
            content: ["コンテンツアクセストークン"],
            help: ["ブラウザに保存されたデータ"],
        },
        {
            type: "check",
            check: ["コンテンツアクセストークンが有効"],
            help: ["CDN によって判定"],
        },
        {
            type: "success",
            action: ["画面の読み込み"],
            help: ["アプリケーションスクリプトが CDN から返される"],
        },
    ],
    data: [docs_authTicket],
}

export const docs_auth: DocsDomain = {
    path: "auth",
    title: "認証・認可",
    usecase: [
        {
            path: "ticket/check",
            title: docs_checkAuthTicket.title,
            purpose: ["業務で必要な時に使用できる", "業務内容をプライベートに保つ"],
            action: [docs_checkAuthTicket, docs_loadApplication],
        },
        {
            path: "ticket/logout",
            title: docs_logout.title,
            purpose: ["業務内容をプライベートに保つ"],
            action: [docs_logout],
        },
        {
            path: "password/authenticate",
            title: docs_authenticatePassword.title,
            purpose: ["業務内容をプライベートに保つ"],
            action: [docs_authenticatePassword, docs_loadApplication],
        },
        {
            path: "password/reset",
            title: docs_resetPassword.title,
            purpose: ["業務内容をプライベートに保つ"],
            action: [docs_requestResetToken, docs_resetPassword, docs_loadApplication],
        },
        {
            path: "password/change",
            title: docs_changePassword.title,
            purpose: ["業務内容をプライベートに保つ"],
            action: [docs_changePassword],
        },
    ],
}

export const docs_auth_legacy: DocsSection[] = [
    docsSection("認証・認可", [
        docsPurpose(["業務で必要な時に使用できる", "業務内容をプライベートに保つ"]),
        docsModule(["認証", "プロフィール", "ユーザー管理"]),
    ]),
]

const docs_auth_profile: DocsSection[] = [
    docsSection_pending("プロフィール", [
        docsPurpose(["業務で必要な時に使用できる", "業務内容をプライベートに保つ"]),
        docsModule(["パスワード変更", "web 証明書再登録"]),
    ]),
]
const docs_auth_user: DocsSection[] = [
    docsSection_pending("ユーザー管理", [
        docsPurpose(["業務で必要な時に使用できる", "業務内容をプライベートに保つ"]),
        docsModule([
            "ユーザーの登録",
            "ユーザーの無効化",
            "ユーザーの削除",
            "ログインID 変更",
            "アクセス権限変更",
            "パスワード変更",
            "web 証明書変更",
        ]),
    ]),
]

export const docs_auth_summary: DocsSection[] = [
    ...docs_auth_sign,
    ...docs_auth_profile,
    ...docs_auth_user,
]

export const docs_auth_detail: DocsSection[][][] = [
    [...docs_auth_sign_action, ...docs_auth_sign_data],
]
