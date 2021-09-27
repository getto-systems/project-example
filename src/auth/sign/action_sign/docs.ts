import {
    docsExplanation,
    docsModule,
    docsNegativeNote,
    docsPurpose,
    docsSection,
    docsSection_double,
    docsSection_pending,
} from "../../../../ui/vendor/getto-application/docs/helper"

import {
    docs_auth_checkAuthTicket,
    docs_auth_checkAuthTicket_description,
} from "../../ticket/action_check/docs"
import {
    docs_auth_logout,
    docs_auth_logout_description,
} from "../../ticket/action_logout/docs"
import {
    docs_auth_authenticatePassword,
    docs_auth_authenticatePassword_description,
} from "../../user/password/action_authenticate/docs"
import {
    docs_auth_resetPassword,
    docs_auth_resetPassword_data,
    docs_auth_resetPassword_description,
} from "../../user/password/reset/docs"

import { docs_auth_ticket } from "../../ticket/docs"
import { docs_auth_loginID } from "../../user/login_id/docs"
import { docs_auth_password } from "../../user/password/docs"

import { DocsSection } from "../../../../ui/vendor/getto-application/docs/data"

export const docs_auth_sign: DocsSection[] = [
    docsSection("認証", [
        docsPurpose(["業務で必要な時に使用できる", "業務内容をプライベートに保つ"]),
        docsModule([
            "認証チケット更新",
            "ログアウト",
            "パスワードログイン",
            "パスワードリセット",
            "web 証明書ログイン",
        ]),
    ]),
]

const docs_auth_authenticateWebAuthn: DocsSection[] = [
    docsSection_pending("web 証明書ログイン", [
        docsModule(["web 証明書で認証", "認証成功でトークンを発行"]),
    ]),
]

export const docs_auth_sign_action: DocsSection[][] = [
    [...docs_auth_checkAuthTicket, ...docs_auth_logout],

    [
        ...docs_auth_authenticatePassword,
        ...docs_auth_resetPassword,
        ...docs_auth_authenticateWebAuthn,
    ],
]

export const docs_auth_sign_data: DocsSection[][] = [
    docs_auth_ticket,
    [...docs_auth_loginID, ...docs_auth_password, ...docs_auth_resetPassword_data],
]

export const docs_auth_sign_explanation: DocsSection[] = [
    docsSection("想定するサーバー・クライアント", [
        docsExplanation(["content-server", "api-server", "http-client", "text-client"]),
    ]),
]

export const docs_auth_sign_negativeNote: DocsSection[] = [
    docsSection_double("判明しているダメな点", [
        docsNegativeNote([
            {
                message: "チケットの有効期限切れの前にチケットを無効化できない",
                help: "最大延長期間を操作することで再認証を促すことは可能",
            },
            {
                message: "チケットが漏れた場合、有効期限延長を続けることで最大期間アクセス可能",
                help:
                    "これをするためには cookie の奪取とメモリの解析を行う必要があるので、事実上不可能",
            },
            {
                message: "http を使用することを想定",
                help: "http 以外の方式で通信する必要が出たときに考える",
            },
            {
                message: "cookie を使用するため別なタブで別ユーザーとしてログインできない",
                help: "別ユーザーでログインする必要がある設計にしない",
            },
        ]),
    ]),
]

export const docs_auth_sign_description: DocsSection[][][] = [
    [docs_auth_checkAuthTicket_description],
    [docs_auth_logout_description],
    [docs_auth_authenticatePassword_description],
    [docs_auth_resetPassword_description],
]
