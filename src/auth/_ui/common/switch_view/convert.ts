import { ConvertLocationResult } from "../../../../z_details/_ui/location/data"

import {
    ResetPasswordVariant,
    ResetPasswordVariantKey,
    SignNav,
    signNavKey,
    StaticSignViewVariant,
    StaticSignViewVariantKey,
} from "../nav/data"

export function detectStaticSignViewVariant(
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

export function detectResetPasswordVariant(
    currentURL: URL,
): ConvertLocationResult<ResetPasswordVariant> {
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
