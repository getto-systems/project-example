export type BoardValue = string & { BoardValue: never }

export const emptyBoardValue: BoardValue = "" as BoardValue

// validate board で参照されているが、変換結果として外に返す構造なので kernel で定義している
export type ConvertBoardResult<T> = Readonly<{ valid: true; value: T }> | Readonly<{ valid: false }>
