import { detectResetPasswordVariant, detectStaticSignViewVariant } from "./convert"

import { ConvertLocationResult } from "../../../../z_details/_ui/location/data"
import { ResetPasswordVariant, StaticSignViewVariant } from "../nav/data"
import { SignViewType } from "./data"

// TODO このファイル名 core.ts を別な名前にしたい
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
