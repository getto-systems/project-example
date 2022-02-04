import { BoardValue } from "../kernel/data"

export interface BoardFieldObserver {
    pin(): void
    peek(): BoardValue
    hasChanged(): boolean
}
