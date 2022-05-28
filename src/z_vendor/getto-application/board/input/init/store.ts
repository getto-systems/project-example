import { BoardValueStore } from "../infra"

export function initBoardValueStore(): Readonly<{
    store: BoardValueStore
    connect: { (setValue: { (value: string): void }): void }
}> {
    const store = new Store()
    return { store, connect: (setValue) => store.connect(setValue) }
}

class Store implements BoardValueStore {
    value: string

    constructor() {
        this.value = ""
    }

    setValue: { (value: string): void } = () => null

    connect(setValue: { (value: string): void }): void {
        setValue(this.get())
        this.setValue = setValue
    }

    get(): string {
        return this.value
    }
    set(value: string): void {
        this.value = value
        this.setValue(this.value)
    }
}
