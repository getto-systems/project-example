import { BoardFieldObserver } from "../infra"

import { BoardValue } from "../../kernel/data"

export function initBoardFieldObserver(value: BoardValueGetter): BoardFieldObserver {
    return new Observer(value)
}

type BoardValueStore = Readonly<{ stored: false }> | Readonly<{ stored: true; value: BoardValue }>

interface BoardValueGetter {
    (): BoardValue
}

class Observer implements BoardFieldObserver {
    value: BoardValueGetter
    store: BoardValueStore = { stored: false }

    constructor(value: BoardValueGetter) {
        this.value = value
    }

    pin(): void {
        this.store = { stored: true, value: this.value() }
    }
    hasChanged(): boolean {
        if (!this.store.stored) {
            return false
        }
        return this.store.value !== this.value()
    }
}
