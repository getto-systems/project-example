import { ConvertBoardFieldResult } from "./data"

// TODO 廃止したい
export interface BoardFieldChecker<T, E> {
    check(): ConvertBoardFieldResult<T, E>
}
