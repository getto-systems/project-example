import { LineIcon, lniClass, lnir } from "../../../../../z_details/_ui/icon/line_icon"

import {
    encodeLocationSearchQuery,
    LocationSearchParam,
} from "../../../../../z_details/_ui/location/helper"

import { SignLinkResource } from "./resource"

import {
    AuthenticatePasswordVariantKey,
    ResetPasswordVariantKey,
    SignNav,
    SignNavHref,
    SignNavItem,
    signNavKey,
    StaticSignViewVariantKey,
} from "../data"

export function initSignLinkResource(): SignLinkResource {
    return {
        link: {
            getNav_static_privacyPolicy: () =>
                markSignNavItem(
                    "プライバシーポリシー",
                    lnir("key-alt"),
                    staticSignViewHref("privacy-policy"),
                ),

            getNav_password_authenticate: () =>
                markSignNavItem(
                    "ログインIDとパスワードでログイン",
                    lnir("arrow-left"),
                    authenticatePasswordHref("authenticate"),
                ),
            getNav_password_reset_requestToken: () =>
                markSignNavItem(
                    "パスワードがわからない方",
                    lnir("question-circle"),
                    resetPasswordHref("request-token", []),
                ),
            getNav_password_reset_requestToken_retry: () =>
                markSignNavItem(
                    "リセットトークン送信からやり直す",
                    lnir("arrow-left"),
                    resetPasswordHref("request-token", []),
                ),
            getHref_password_reset_checkStatus: (sessionID) =>
                resetPasswordHref("check-status", [
                    [signNavKey(SignNav.passwordResetSessionID), sessionID],
                ]),
        },
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
    params: LocationSearchParam[],
): SignNavHref {
    return href([[signNavKey(SignNav.passwordReset), variant], ...params])
}
function href(params: LocationSearchParam[]): SignNavHref {
    return `?${encodeLocationSearchQuery(params)}` as SignNavHref
}

function markSignNavItem(label: string, icon: LineIcon, href: SignNavHref): SignNavItem {
    return { label, icon: lniClass(icon), href } as SignNavItem
}
