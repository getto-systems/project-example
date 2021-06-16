import { FetchRepositoryResult, StoreRepositoryResult } from "../infra"

export interface MemoryDB<T> {
    get(): Promise<FetchRepositoryResult<T>>
    set(value: T): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}

export function initMemoryDB<T>(): MemoryDB<T> {
    let store: FetchRepositoryResult<T> = { success: true, found: false }

    return {
        get: async () => store,
        set: async (value) => {
            store = { success: true, found: true, value }
            return { success: true }
        },
        remove: async () => {
            store = { success: true, found: false }
            return { success: true }
        },
    }
}
