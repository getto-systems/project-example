import { ValidateBoardStack, ValidateBoardStateFound } from "../infra"

export function initValidateBoardStack(): ValidateBoardStack {
    const stack: Map<string, boolean> = new Map()

    return {
        get(name: string): ValidateBoardStateFound {
            const state = stack.get(name)
            if (state === undefined) {
                return { found: false }
            }
            return { found: true, state }
        },
        set(name: string, valid: boolean): void {
            stack.set(name, valid)
        },
        delete(name: string): void {
            stack.delete(name)
        },
    }
}
