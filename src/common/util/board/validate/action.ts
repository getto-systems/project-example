import { Atom, combineAtom, composeAtom, mapAtom } from "../../../../z_vendor/getto-atom/atom"
import { LoadState, mapLoadState } from "../../load/data"

import { SelectBoardValueError, ValidateBoardValue } from "./data"

export type ValidateBoardState = { valid: boolean }

export function initValidateBoardAtom<E>(
    values: readonly Atom<ValidateBoardValue<unknown, E>>[],
): Atom<ValidateBoardState> {
    return composeAtom(values, (resultArr) => {
        for (const result of resultArr) {
            if (!result.valid) {
                return { valid: false }
            }
        }
        return { valid: true }
    })
}

export function initValidateTextBoardValueAtom<T, E>(
    action: {
        value: Atom<string>
    },
    infra: Readonly<{
        convert: { (value: string): ValidateBoardValue<T, E> }
    }>,
): Atom<ValidateBoardValue<T, E>> {
    return mapAtom(action.value, infra.convert)
}

export function initValidateSelectBoardValueAtom<T>(
    action: {
        value: Atom<string>
    },
    options: Atom<LoadState<readonly T[]>>,
    infra: Readonly<{
        convert: { (value: T): string }
    }>,
): Atom<ValidateBoardValue<T, SelectBoardValueError>> {
    const optionsAtom = mapOptionsAtom(options, infra)

    return combineAtom(action.value, optionsAtom, (value, options) => {
        return convert(value, options)
    })

    function convert(
        value: string,
        options: LoadState<readonly Readonly<{ string: string; value: T }>[]>,
    ): ValidateBoardValue<T, SelectBoardValueError> {
        if (!options.isLoad) {
            return { valid: false, err: { type: "not-loaded" } }
        }

        const found = options.data.find((option) => option.string === value)
        if (found === undefined) {
            return { valid: false, err: { type: "not-selected" } }
        }

        return { valid: true, value: found.value }
    }
}

export function initValidateMultipleBoardValueAtom<T>(
    action: {
        value: Atom<readonly string[]>
    },
    options: Atom<LoadState<readonly T[]>>,
    infra: Readonly<{
        convert: { (value: T): string }
    }>,
): Atom<ValidateBoardValue<readonly T[], SelectBoardValueError>> {
    const optionsAtom = mapOptionsAtom(options, infra)
    return combineAtom(action.value, optionsAtom, (value, options) => {
        return convert(value, options)
    })

    function convert(
        value: readonly string[],
        options: LoadState<readonly Readonly<{ string: string; value: T }>[]>,
    ): ValidateBoardValue<readonly T[], SelectBoardValueError> {
        if (!options.isLoad) {
            return { valid: false, err: { type: "not-loaded" } }
        }

        const result: T[] = []
        for (const item of value) {
            const found = options.data.find((option) => option.string === item)
            if (found === undefined) {
                return { valid: false, err: { type: "not-selected" } }
            }

            result.push(found.value)
        }

        return { valid: true, value: result }
    }
}

export function initValidateVectorBoardValueAtom<T, E>(
    action: {
        value: Atom<readonly string[]>
    },
    infra: Readonly<{
        convert: { (value: string): ValidateBoardValue<T, E> }
    }>,
): Atom<ValidateBoardValue<readonly T[], readonly E[]>> {
    return mapAtom(action.value, convert)

    function convert(value: readonly string[]): ValidateBoardValue<readonly T[], readonly E[]> {
        const result: T[] = []
        const errors: E[] = []

        for (const item of value) {
            const converted = infra.convert(item)
            if (converted.valid) {
                result.push(converted.value)
            } else {
                errors.push(converted.err)
            }
        }

        return errors.length > 0 ? { valid: false, err: errors } : { valid: true, value: result }
    }
}

function mapOptionsAtom<T>(
    options: Atom<LoadState<readonly T[]>>,
    infra: Readonly<{
        convert: { (value: T): string }
    }>,
): Atom<LoadState<readonly Readonly<{ value: T; string: string }>[]>> {
    return mapAtom(options, (values) => {
        return mapLoadState(values, (values) =>
            values.map((value) => ({
                value,
                string: infra.convert(value),
            })),
        )
    })
}
