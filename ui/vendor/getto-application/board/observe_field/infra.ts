import { BoardValue } from "../kernel/data";

export type ObserveBoardFieldInfra = Readonly<{
    observer: BoardFieldObserver
}>

export interface BoardFieldObserver {
    pin(): void
    peek(): BoardValue
    hasChanged(): boolean
}
