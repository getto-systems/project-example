// TODO 廃止したい
export type BoardValue = string & { BoardValue: never }

export const emptyBoardValue: BoardValue = "" as BoardValue
export const zeroBoardValue: BoardValue = "0" as BoardValue

export type ConvertBoardResult<T> = Readonly<{ valid: true; value: T }> | Readonly<{ valid: false }>
