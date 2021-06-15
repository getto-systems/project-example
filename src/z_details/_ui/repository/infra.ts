import { RepositoryError } from "./data"

export interface RepositoryPod<V, R> {
    (converter: RepositoryConverter<V, R>): Repository<V>
}

export interface RepositoryConverter<V, R> {
    toRepository(value: V): R
    fromRepository(raw: R): ConvertRepositoryResult<V>
}

export interface Repository<T> {
    get(): Promise<FetchRepositoryResult<T>>
    set(value: T): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}

export type FetchRepositoryResult<T> =
    | Readonly<{ success: true; found: true; value: T }>
    | FetchRepositoryNotFoundResult
    | RepositoryErrorResult

export type FetchRepositoryRemovedResult = FetchRepositoryNotFoundResult | RepositoryErrorResult

export type FetchRepositoryNotFoundResult = Readonly<{ success: true; found: false }>

export type StoreRepositoryResult = Readonly<{ success: true }> | RepositoryErrorResult

export type RepositoryErrorResult = Readonly<{ success: false; err: RepositoryError }>

export type ConvertRepositoryResult<T> =
    | Readonly<{ valid: true; value: T }>
    | Readonly<{ valid: false }>
