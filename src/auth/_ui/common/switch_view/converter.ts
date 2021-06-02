import { LocationConverter } from "../../../../../ui/vendor/getto-application/location/infra"

import {
    ResetPasswordVariant,
    ResetPasswordVariantKey,
    StaticSignViewVariant,
    StaticSignViewVariantKey,
} from "../nav/data"

type StaticSignViewVariantConverter = LocationConverter<StaticSignViewVariant, string | null>
export const staticSignViewVariantLocationConverter: StaticSignViewVariantConverter = (search) => {
    if (!search) {
        return { valid: false }
    }
    if (search in StaticSignViewVariant) {
        // search が StaticSignViewVariant のメンバーなら、string は StaticSignViewVariantKey である
        return { valid: true, value: StaticSignViewVariant[search as StaticSignViewVariantKey] }
    }
    return { valid: false }
}

type ResetPasswordVariantConverter = LocationConverter<ResetPasswordVariant, string | null>
export const resetPasswordVariantLocationConverter: ResetPasswordVariantConverter = (search) => {
    if (!search) {
        return { valid: false }
    }
    if (search in ResetPasswordVariant) {
        // search が ResetPasswordVariant のメンバーなら、string は ResetPasswordVariantKey である
        return { valid: true, value: ResetPasswordVariant[search as ResetPasswordVariantKey] }
    }
    return { valid: false }
}
