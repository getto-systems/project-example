import { ConvertBoardFieldResult } from "./data"

export type ValidateBoardFieldInfra<T, E> = Readonly<{
    converter: BoardFieldConverter<T, E>
}>

export interface BoardFieldChecker<T, E> {
    get(): ConvertBoardFieldResult<T, E>
    check(): void
}

export interface BoardFieldConverter<T, E> {
    (): ConvertBoardFieldResult<T, E>
}

export interface BoardFieldValueConverter<T, V, E> {
    (value: V): ConvertBoardFieldResult<T, E>
}
