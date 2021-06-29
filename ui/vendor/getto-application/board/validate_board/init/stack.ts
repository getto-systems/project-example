import { ValidateBoardStack, ValidateBoardStateFound } from "../infra"

export function initValidateBoardStack(): ValidateBoardStack {
    return new Stack()
}

class Stack implements ValidateBoardStack {
    stack: Map<string, boolean> = new Map()

    get(name: string): ValidateBoardStateFound {
        const state = this.stack.get(name)
        if (state === undefined) {
            return { found: false }
        }
        return { found: true, state }
    }
    set(name: string, valid: boolean): void {
        this.stack.set(name, valid)
    }
}
