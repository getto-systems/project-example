export interface DB<T> {
    get(): Promise<FetchDBResult<T>>
    set(value: T): Promise<StoreDBResult>
    remove(): Promise<StoreDBResult>
}

export type FetchDBResult<T> =
    | Readonly<{ success: true; found: true; value: T }>
    | Readonly<{ success: true; found: false }>
    | DBErrorResult

export type StoreDBResult = Readonly<{ success: true }> | DBErrorResult

export type DBErrorResult = Readonly<{ success: false; err: DBError }>

export type DBError = Readonly<{ type: "infra-error"; err: string }>

export interface ToDBConverter<T> {
    (value: T): string
}
export interface FromDBConverter<T> {
    (raw: string): T
}
