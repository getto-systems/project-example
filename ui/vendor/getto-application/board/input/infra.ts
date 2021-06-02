import { BoardValue } from "../kernel/data"

export interface BoardValueStore {
    get(): BoardValue
    set(value: BoardValue): void
}
