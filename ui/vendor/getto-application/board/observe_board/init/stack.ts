import { ObserveBoardStack, ObserveBoardStateFound } from "../infra"

export function initObserveBoardStack(): ObserveBoardStack {
    return new Stack()
}

class Stack implements ObserveBoardStack {
    stack: Map<string, boolean> = new Map()

    get(name: string): ObserveBoardStateFound {
        const hasChanged = this.stack.get(name)
        if (hasChanged === undefined) {
            return { found: false }
        }
        return { found: true, hasChanged }
    }
    set(name: string, hasChanged: boolean): void {
        this.stack.set(name, hasChanged)
    }
}
