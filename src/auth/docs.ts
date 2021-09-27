import { docs_auth_sign, docs_auth_sign_action, docs_auth_sign_data } from "./_ui/action_sign/docs"
import { docs_checkAuthTicket } from "./ticket/action_check/docs"

import {
    docsUsecase,
    docsDomain,
    docsModule,
    docsPurpose,
    docsSection,
    docsSection_pending,
    docsAction,
    docsPath,
} from "../../ui/vendor/getto-application/docs/helper"

import {
    DocsUsecase,
    DocsSection,
    DocsUsecaseDescription,
} from "../../ui/vendor/getto-application/docs/data"
import { docs_authTicket } from "./ticket/docs"
import { docs_authenticatePassword } from "./user/password/action_authenticate/docs"
import { docs_loginID } from "./user/login_id/docs"
import { docs_password } from "./user/password/docs"
import { docs_authUser } from "./user/docs"
import { docs_logout } from "./ticket/action_logout/docs"
import { docs_requestResetToken } from "./user/password/reset/action_request_token/docs"
import { docs_reset } from "./user/password/reset/docs"
import { docs_resetPassword } from "./user/password/reset/action_reset/docs"

export const docs_auth = docsDomain<AuthUsecase, AuthAction, AuthData>(
    "認証・認可",
    ["業務で必要な時に使用できる", "業務内容をプライベートに保つ"],
    ["authTicket/check", "password/authenticate", "password/reset", "logout"],
    (name) => usecase[name],
)

const usecase = {
    "authTicket/check": docsAuthUsecase(
        "authTicket/check",
        ["業務で必要な時に使用できる", "業務内容をプライベートに保つ"],
        { action: ["authTicket/check", "loadApplication"], data: ["authUser", "authTicket"] },
    ),
    "password/authenticate": docsAuthUsecase(
        "password/authenticate",
        ["業務内容をプライベートに保つ"],
        {
            action: ["password/authenticate", "loadApplication"],
            data: ["authUser", "authTicket", "loginID", "password"],
        },
    ),
    "password/reset": docsAuthUsecase("password/reset", ["業務で必要な時に使用できる"], {
        action: ["password/reset/requestResetToken", "password/reset", "loadApplication"],
        data: ["authUser", "loginID", "password", "reset"],
    }),
    logout: docsAuthUsecase("logout", ["業務内容をプライベートに保つ"], {
        action: ["logout"],
        data: ["authUser", "authTicket"],
    }),
} as const

const action = {
    "authTicket/check": docs_checkAuthTicket,
    "password/authenticate": docs_authenticatePassword,
    "password/reset/requestResetToken": docs_requestResetToken,
    "password/reset": docs_resetPassword,
    logout: docs_logout,
    loadApplication: docsAction("アプリケーションのロード", ({ item }) => [
        item("input", ["コンテンツアクセストークン"], ["ブラウザに保存されたデータ"]),
        item("check", ["コンテンツアクセストークンが有効"], ["CDN によって判定"]),
        item("success", ["画面の読み込み"], ["アプリケーションスクリプトが CDN から返される"]),
    ]),
} as const

const data = {
    authTicket: docs_authTicket,
    authUser: docs_authUser,
    loginID: docs_loginID,
    password: docs_password,
    reset: docs_reset,
} as const

export type AuthUsecase = keyof typeof usecase
export type AuthAction = keyof typeof action
export type AuthData = keyof typeof data

function docsAuthUsecase(
    title: AuthAction,
    purpose: string[],
    content: DocsUsecaseDescription<AuthAction, AuthData>,
): DocsUsecase<AuthAction, AuthData> {
    return docsUsecase(docsPath(title), title, purpose, content, {
        toAction: (name) => action[name],
        toData: (name) => data[name],
    })
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
