import { ConvertBoardResult } from "./data"

// TODO 廃止したい
export interface BoardConverter<T> {
    (): ConvertBoardResult<T>
}
