import { ConvertBoardResult } from "./data"

export interface BoardConverter<T> {
    (): ConvertBoardResult<T>
}
