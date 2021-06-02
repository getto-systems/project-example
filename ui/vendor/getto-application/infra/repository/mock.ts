import {
    FetchRepositoryResult,
    Repository,
    StoreRepositoryResult,
} from "./infra"

export function mockRepository<T>(): Repository<T> {
    return new DB()
}

class DB<T> implements Repository<T> {
    store: FetchRepositoryResult<T> = { success: true, found: false }

    async get(): Promise<FetchRepositoryResult<T>> {
        return this.store
    }
    async set(value: T): Promise<StoreRepositoryResult> {
        this.store = { success: true, found: true, value }
        return { success: true }
    }
    async remove(): Promise<StoreRepositoryResult> {
        this.store = { success: true, found: false }
        return { success: true }
    }
}
