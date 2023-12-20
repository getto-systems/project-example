export type ValidateBoardValue<T, E> =
    | Readonly<{ valid: true; value: T }>
    | Readonly<{ valid: false; err: E }>

export type SelectBoardValueError =
    | Readonly<{ type: "not-loaded" }>
    | Readonly<{ type: "not-selected" }>
    | Readonly<{ type: "invalid-selected" }>
