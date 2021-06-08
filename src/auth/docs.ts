import { docs_auth_sign, docs_auth_sign_action, docs_auth_sign_data } from "./_ui/action_sign/docs"
import { docs_checkAuthTicket } from "./auth_ticket/_ui/action_check/docs"

import {
    docsUsecase,
    docsDomain,
    docsModule,
    docsPurpose,
    docsSection,
    docsSection_pending,
    docsAction,
} from "../../ui/vendor/getto-application/docs/helper"

import {
    DocsUsecase,
    DocsSection,
    DocsUsecaseDescription,
} from "../../ui/vendor/getto-application/docs/data"
import { docs_authTicket } from "./auth_ticket/docs"

export const docs_auth = docsDomain<AuthUsecase, AuthAction, AuthData>(
    "認証・認可",
    ["業務で必要な時に使用できる", "業務内容をプライベートに保つ"],
    ["checkAuthTicket"],
    (name) => usecase[name],
)

const usecase = {
    checkAuthTicket: docsAuthUsecase(
        "checkAuthTicket",
        ["業務で必要な時に使用できる", "業務内容をプライベートに保つ"],
        { action: ["checkAuthTicket", "loadApplication"], data: ["authTicket"] },
    ),
} as const

const action = {
    checkAuthTicket: docs_checkAuthTicket,
    loadApplication: docsAction("アプリケーションのロード", ({ item }) => [
        item("input", ["コンテンツアクセストークン"], ["ブラウザに保存されたデータ"]),
        item("check", ["コンテンツアクセストークンが有効"], ["CDN によって判定"]),
        item("success", ["画面の読み込み"], ["アプリケーションスクリプトが CDN から返される"]),
    ]),
} as const

const data = {
    authTicket: docs_authTicket,
} as const

export type AuthUsecase = keyof typeof usecase
export type AuthAction = keyof typeof action
export type AuthData = keyof typeof data

function docsAuthUsecase(
    title: AuthAction,
    purpose: string[],
    content: DocsUsecaseDescription<AuthAction, AuthData>,
): DocsUsecase<AuthAction, AuthData> {
    return docsUsecase(title, purpose, content, {
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
