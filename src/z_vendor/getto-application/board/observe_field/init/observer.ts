import { BoardFieldObserver } from "../infra"

export function initBoardFieldObserver<V>(
    value: Readonly<{
        current: { (): V }
        isSame?: { (a: V, b: V): boolean }
    }>,
): BoardFieldObserver {
    return new Observer(value)
}

interface ObserveValueGetter<V> {
    (): V
}
interface ObserveValueCompare<V> {
    (a: V, b: V): boolean
}

class Observer<V> implements BoardFieldObserver {
    current: ObserveValueGetter<V>
    isSame: ObserveValueCompare<V> = (a, b) => a === b
    store: V

    constructor({
        current,
        isSame,
    }: Readonly<{
        current: { (): V }
        isSame?: { (a: V, b: V): boolean }
    }>) {
        this.current = current
        if (isSame) {
            this.isSame = isSame
        }
        this.store = current()
    }

    pin(): void {
        this.store = this.current()
    }
    hasChanged(): boolean {
        return !this.isSame(this.store, this.current())
    }
}
