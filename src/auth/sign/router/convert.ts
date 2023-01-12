import { ConvertLocationResult } from "../../../common/util/location/data"

import {
    ResetPasswordVariant,
    ResetPasswordVariantKey,
    SignNav,
    signNavKey,
    StaticSignViewVariant,
    StaticSignViewVariantKey,
} from "../nav/data"

import { SignViewType } from "./data"

export function detectSignViewType(currentURL: URL): ConvertLocationResult<SignViewType> {
    const staticView = detectStaticSignViewVariant(currentURL)
    if (staticView.valid) {
        return { valid: true, value: staticViewType(staticView.value) }
    }

    const resetPassword = detectResetPasswordVariant(currentURL)
    if (resetPassword.valid) {
        return { valid: true, value: resetPasswordViewType(resetPassword.value) }
    }

    return { valid: false }
}

function detectStaticSignViewVariant(
    currentURL: URL,
): ConvertLocationResult<StaticSignViewVariant> {
    const search = currentURL.searchParams.get(signNavKey(SignNav.static))
    if (!search) {
        return { valid: false }
    }
    if (search in StaticSignViewVariant) {
        // search が StaticSignViewVariant のメンバーなら、string は StaticSignViewVariantKey である
        return { valid: true, value: StaticSignViewVariant[search as StaticSignViewVariantKey] }
    }
    return { valid: false }
}

function detectResetPasswordVariant(currentURL: URL): ConvertLocationResult<ResetPasswordVariant> {
    const search = currentURL.searchParams.get(signNavKey(SignNav.passwordReset))
    if (!search) {
        return { valid: false }
    }
    if (search in ResetPasswordVariant) {
        // search が ResetPasswordVariant のメンバーなら、string は ResetPasswordVariantKey である
        return { valid: true, value: ResetPasswordVariant[search as ResetPasswordVariantKey] }
    }
    return { valid: false }
}

function staticViewType(variant: StaticSignViewVariant): SignViewType {
    switch (variant) {
        case StaticSignViewVariant["privacy-policy"]:
            return "static-privacyPolicy"
    }
}
function resetPasswordViewType(variant: ResetPasswordVariant): SignViewType {
    switch (variant) {
        case ResetPasswordVariant["request-token"]:
            return "password-reset-requestToken"

        case ResetPasswordVariant["reset"]:
            return "password-reset"
    }
}
