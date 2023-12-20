import { SingleBoardStore } from "../infra"

export function initSingleBoardStore(): Readonly<{
    store: SingleBoardStore
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
