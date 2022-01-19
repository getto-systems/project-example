import { fetchRepositoryRemovedResult, mapFetchRepositoryResult } from "./helper"

import { FetchRepositoryResult, RepositoryConverter, StoreRepositoryResult } from "../infra"

export function convertDB<V, R>(db: DB<R>, converter: RepositoryConverter<V, R>): DB<V> {
    return {
        get: () =>
            mapFetchRepositoryResult(db.get(), async (value) => {
                const result = converter.fromRepository(value)
                if (!result.valid) {
                    return fetchRepositoryRemovedResult(await db.remove())
                }
                return { success: true, found: true, value: result.value }
            }),
        set: (value) => db.set(converter.toRepository(value)),
        remove: () => db.remove(),
    }
}

interface DB<T> {
    get(): Promise<FetchRepositoryResult<T>>
    set(value: T): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}
