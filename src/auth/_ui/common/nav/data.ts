export type SignNavHref = string & { SignNavHref: never }
export type SignNavItem = { SignNavItem: never } & Readonly<{
    label: string
    icon: string
    href: SignNavHref
}>

export enum SignNav {
    "static",
    "passwordAuthenticate",
    "passwordReset",
    "passwordResetSessionID",
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

        case SignNav.passwordResetSessionID:
            return "-password-reset-session-id"

        case SignNav.passwordResetToken:
            return "-password-reset-token"
    }
}

export enum StaticSignViewVariant {
    "privacy-policy",
}
export type StaticSignViewVariantKey = keyof typeof StaticSignViewVariant

export enum AuthenticatePasswordVariant {
    "authenticate",
}
export type AuthenticatePasswordVariantKey = keyof typeof AuthenticatePasswordVariant

export enum ResetPasswordVariant {
    "request-token",
    "check-status",
    "reset",
}
export type ResetPasswordVariantKey = keyof typeof ResetPasswordVariant
