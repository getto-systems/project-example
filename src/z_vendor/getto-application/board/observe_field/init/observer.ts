import { BoardFieldObserver } from "../infra"

export function initBoardFieldObserver<V>(value: ObserveValueGetter<V>): BoardFieldObserver {
    return new Observer(value)
}

type ObserveValueStore<V> = Readonly<{ stored: false }> | Readonly<{ stored: true; value: V }>

interface ObserveValueGetter<V> {
    (): V
}

class Observer<V> implements BoardFieldObserver {
    value: ObserveValueGetter<V>
    store: ObserveValueStore<V> = { stored: false }

    constructor(value: ObserveValueGetter<V>) {
        this.value = value
    }

    pin(): void {
        this.store = { stored: true, value: this.value() }
    }
    peek(): V {
        if (!this.store.stored) {
            return this.value()
        }
        return this.store.value
    }
    hasChanged(): boolean {
        if (!this.store.stored) {
            return false
        }
        return this.store.value !== this.value()
    }
}
