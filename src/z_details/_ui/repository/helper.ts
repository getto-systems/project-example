import {
    FetchRepositoryResult,
    StoreRepositoryResult,
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
