export type ConvertBoardResult<T> = Readonly<{ valid: true; value: T }> | Readonly<{ valid: false }>
