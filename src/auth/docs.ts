import { docs_checkAuthTicket } from "./ticket/check/docs"
import { docs_authTicket } from "./ticket/docs"
import { docs_authenticatePassword } from "./user/password/authenticate/docs"
import { docs_logout } from "./ticket/logout/docs"
import { docs_requestResetToken } from "./user/password/reset/request_token/docs"
import { docs_resetPassword } from "./user/password/reset/reset/docs"
import { docs_changePassword } from "./user/password/change/docs"

import { DocsAction, DocsDomain } from "../z_vendor/getto-application/docs/data"

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
