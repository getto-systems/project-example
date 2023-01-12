export type PrepareElementState<T> =
    | Readonly<{ isLoad: false }>
    | Readonly<{ isLoad: true; data: T }>

export function prepared<T>(data: T): PrepareElementState<T> {
    return { isLoad: true, data }
}
export function preparing<T>(): PrepareElementState<T> {
    return { isLoad: false }
}
