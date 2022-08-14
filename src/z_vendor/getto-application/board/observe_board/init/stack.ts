import { ObserveBoardStack, ObserveBoardStateFound } from "../infra"

export function initObserveBoardStack(): ObserveBoardStack {
    const stack: Map<string, boolean> = new Map()

    return {
        get(name: string): ObserveBoardStateFound {
            const hasChanged = stack.get(name)
            if (hasChanged === undefined) {
                return { found: false }
            }
            return { found: true, hasChanged }
        },
        set(name: string, hasChanged: boolean): void {
            stack.set(name, hasChanged)
        },
        clear(): void {
            stack.clear()
        },
    }
}
