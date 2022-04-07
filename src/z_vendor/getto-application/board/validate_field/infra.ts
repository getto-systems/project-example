import { ConvertBoardFieldResult } from "./data"

export interface BoardFieldChecker<T, E> {
    check(): ConvertBoardFieldResult<T, E>
}

export interface BoardFieldConverter<T, E> {
    (): ConvertBoardFieldResult<T, E>
}

// TODO 廃止したほうがいいと思う
export interface BoardFieldValueConverter<T, V, E> {
    (value: V): ConvertBoardFieldResult<T, E>
}
