import { lnir } from "../../../common/util/icon/detail/line_icon"
import {
    encodeLocationSearchQuery,
    LocationSearchParam,
} from "../../../common/util/location/helper"

import { Icon } from "../../../common/util/icon/data"
import {
    AuthenticatePasswordVariantKey,
    ResetPasswordVariantKey,
    SignNav,
    SignNavHref,
    SignNavItem,
    signNavKey,
    StaticSignViewVariantKey,
} from "./data"

export interface SignLink {
    getNav_static_privacyPolicy(): SignNavItem

    getNav_password_authenticate(): SignNavItem
    getNav_password_reset_requestToken(): SignNavItem
    getNav_password_reset_requestToken_retry(): SignNavItem
}

export function initSignLink(): SignLink {
    return {
        getNav_static_privacyPolicy: () =>
            markSignNavItem(
                "プライバシーポリシー",
                lnir(["key-alt"]),
                staticSignViewHref("privacy-policy"),
            ),

        getNav_password_authenticate: () =>
            markSignNavItem(
                "ログインIDとパスワードでログイン",
                lnir(["arrow-left"]),
                authenticatePasswordHref("authenticate"),
            ),
        getNav_password_reset_requestToken: () =>
            markSignNavItem(
                "パスワードがわからない方",
                lnir(["question-circle"]),
                resetPasswordHref("request-token", []),
            ),
        getNav_password_reset_requestToken_retry: () =>
            markSignNavItem(
                "リセットトークン送信からやり直す",
                lnir(["arrow-left"]),
                resetPasswordHref("request-token", []),
            ),
    }
}

function staticSignViewHref(variant: StaticSignViewVariantKey): SignNavHref {
    return href([[signNavKey(SignNav.static), variant]])
}
function authenticatePasswordHref(variant: AuthenticatePasswordVariantKey): SignNavHref {
    return href([[signNavKey(SignNav.passwordAuthenticate), variant]])
}
function resetPasswordHref(
    variant: ResetPasswordVariantKey,
    params: readonly LocationSearchParam[],
): SignNavHref {
    return href([[signNavKey(SignNav.passwordReset), variant], ...params])
}
function href(params: readonly LocationSearchParam[]): SignNavHref {
    return `?${encodeLocationSearchQuery(params)}` as SignNavHref
}

function markSignNavItem(label: string, icon: Icon, href: SignNavHref): SignNavItem {
    return { label, icon, href } as SignNavItem
}
