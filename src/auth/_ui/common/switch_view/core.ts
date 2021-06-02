import {
    ResetPasswordVariant,
    SignNav,
    signNavKey,
    StaticSignViewVariant,
} from "../nav/data"
import { SignViewLocationDetectMethod, SignViewType } from "./data"
import {
    resetPasswordVariantLocationConverter,
    staticSignViewVariantLocationConverter,
} from "./converter"

export const detectSignViewType: SignViewLocationDetectMethod = (currentURL) => {
    const staticView = staticSignViewVariantLocationConverter(
        currentURL.searchParams.get(signNavKey(SignNav.static)),
    )
    if (staticView.valid) {
        return { valid: true, value: staticViewType(staticView.value) }
    }

    const resetPassword = resetPasswordVariantLocationConverter(
        currentURL.searchParams.get(signNavKey(SignNav.passwordReset)),
    )
    if (resetPassword.valid) {
        return { valid: true, value: resetPasswordViewType(resetPassword.value) }
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

        case ResetPasswordVariant["check-status"]:
            return "password-reset-checkStatus"

        case ResetPasswordVariant["reset"]:
            return "password-reset"
    }
}
