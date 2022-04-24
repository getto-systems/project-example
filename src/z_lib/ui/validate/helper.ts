import { Validator } from "./infra"

import { ConvertBoardFieldResult } from "../../../z_vendor/getto-application/board/validate_field/data"

export function converter<T, V, E>(
    map: { (value: T): V },
    validators: readonly Validator<T, E>[],
): { (value: T): ConvertBoardFieldResult<V, readonly E[]> } {
    return (value) => {
        const err: E[] = []
        validators.forEach((validator) => {
            const result = validator(value)
            if (!result.valid) {
                err.push(result.err)
            }
        })
        if (err.length > 0) {
            return { valid: false, err }
        }
        return { valid: true, value: map(value) }
    }
}
