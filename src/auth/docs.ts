import {
    docsModule,
    docsPurpose,
    docsSection,
    docsSection_pending,
} from "../../ui/vendor/getto-application/docs/helper"

import { DocsSection } from "../../ui/vendor/getto-application/docs/data"
import { docs_auth_sign, docs_auth_sign_action, docs_auth_sign_data } from "./_ui/action_sign/docs"

export const docs_auth: DocsSection[] = [
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
