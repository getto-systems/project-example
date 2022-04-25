// TODO kernel/data の ConvertBoardResult と名前が紛らわしいのでなんとかしたい
export type ConvertBoardFieldResult<T, E> =
    | Readonly<{ valid: true; value: T }>
    | Readonly<{ valid: false; err: E }>

export type ValidateBoardFieldResult<E> =
    | Readonly<{ valid: true }>
    | Readonly<{ valid: false; err: E }>
