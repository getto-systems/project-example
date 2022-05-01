import { Icon } from "../../../z_lib/ui/icon/data"

export type SignNavHref = string & { SignNavHref: never }
export type SignNavItem = { SignNavItem: never } & Readonly<{
    label: string
    icon: Icon
    href: SignNavHref
}>

export enum SignNav {
    "static",
    "passwordAuthenticate",
    "passwordReset",
    "passwordResetToken",
}

export function signNavKey(nav: SignNav): string {
    // ログイン前画面ではハイフンから始まるクエリを使用する
    switch (nav) {
        case SignNav.static:
            return "-static"

        case SignNav.passwordAuthenticate:
            return "-password-authenticate"

        case SignNav.passwordReset:
            return "-password-reset"

        case SignNav.passwordResetToken:
            return "-password-reset-token"
    }
}

export enum StaticSignViewVariant {
    "privacy-policy",
}
export type StaticSignViewVariantKey = keyof typeof StaticSignViewVariant

export type AuthenticatePasswordVariantKey = ["authenticate"]

export enum ResetPasswordVariant {
    "request-token",
    "reset",
}
export type ResetPasswordVariantKey = keyof typeof ResetPasswordVariant
