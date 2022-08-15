import { BoardValueStore } from "../infra"

export function initBoardValueStore(): Readonly<{
    store: BoardValueStore
    connect: { (setValue: { (value: string): void }): void }
}> {
    let value = ""
    let setValue: { (value: string): void } = () => null

    return {
        store: {
            get(): string {
                return value
            },
            set(newValue: string): void {
                value = newValue
                setValue(newValue)
            },
        },
        connect: (newSetValue) => {
            newSetValue(value)
            setValue = newSetValue
        },
    }
}
