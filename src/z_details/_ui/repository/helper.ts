import {
    RepositoryConverter,
    FetchRepositoryResult,
    StoreRepositoryResult,
    Repository,
    RepositoryPod,
    RepositoryErrorResult,
    FetchRepositoryRemovedResult,
} from "./infra"

export function fetchRepositoryRemovedResult(
    result: StoreRepositoryResult,
): FetchRepositoryRemovedResult {
    if (!result.success) {
        return result
    }
    return { success: true, found: false }
}

export async function mapFetchRepositoryResult<V, R>(
    promise: Promise<FetchRepositoryResult<R>>,
    convert: { (value: R): Promise<FetchRepositoryResult<V>> },
): Promise<FetchRepositoryResult<V>> {
    const result = await promise
    if (!result.success || !result.found) {
        return result
    }
    return convert(result.value)
}

export function repositoryError(err: unknown): RepositoryErrorResult {
    return { success: false, err: { type: "infra-error", err: `${err}` } }
}

export function convertRepository<V, R>(db: Repository<R>): RepositoryPod<V, R> {
    return (converter) => new Converter(db, converter)
}

class Converter<V, R> implements Repository<V> {
    db: Repository<R>
    converter: RepositoryConverter<V, R>

    constructor(db: Repository<R>, converter: RepositoryConverter<V, R>) {
        this.db = db
        this.converter = converter
    }

    async get(): Promise<FetchRepositoryResult<V>> {
        const raw = await this.db.get()
        if (!raw.success || !raw.found) {
            return raw
        }

        const convertResult = this.converter.fromRepository(raw.value)
        if (!convertResult.valid) {
            return { success: true, found: false }
        }

        return { success: true, found: true, value: convertResult.value }
    }
    set(value: V): Promise<StoreRepositoryResult> {
        return this.db.set(this.converter.toRepository(value))
    }
    remove(): Promise<StoreRepositoryResult> {
        return this.db.remove()
    }
}
