export type SelectResult<T> =
    | Readonly<{ isSelected: false }>
    | Readonly<{ isSelected: true; value: T }>

export type ValidateResult<E> = Readonly<{ valid: true }> | Readonly<{ valid: false; err: E }>

export type ValidateTextError =
    | Readonly<{ type: "empty" }>
    | Readonly<{ type: "too-long"; maxLength: number }>
    | Readonly<{ type: "invalid-email" }>

export type ValidateSelectError = Readonly<{ type: "not-selected" }>
