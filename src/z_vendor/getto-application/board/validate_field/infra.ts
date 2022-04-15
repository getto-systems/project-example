import { ConvertBoardFieldResult } from "./data"

export interface BoardFieldChecker<T, E> {
    check(): ConvertBoardFieldResult<T, E>
}
