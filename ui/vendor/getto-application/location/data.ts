export type ConvertLocationResult<T> =
    | Readonly<{ valid: true; value: T }>
    | Readonly<{ valid: false }>
