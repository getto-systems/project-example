export type LoadState<T> = Readonly<{ isLoad: false }> | Readonly<{ isLoad: true; data: T }>

export function loadState_loaded<T>(data: T): LoadState<T> {
    return { isLoad: true, data }
}
export function loadState_loading<T>(): LoadState<T> {
    return { isLoad: false }
}

export function mapLoadState<T, M>(state: LoadState<T>, mapper: (value: T) => M) {
    if (!state.isLoad) {
        return loadState_loading<M>()
    }
    return loadState_loaded(mapper(state.data))
}
