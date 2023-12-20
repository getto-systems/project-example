export type SingleBoardStore = WritableBoardStore<string>
export type MultipleBoardStore = WritableBoardStore<readonly string[]>
export type FileBoardStore = ReadonlyBoardStore<SelectFileResult>

export interface WritableBoardStore<T> {
    get(): T
    set(value: T): void
}
export interface ReadonlyBoardStore<T> {
    get(): T
}

export type SelectFileResult = Readonly<{ found: false }> | Readonly<{ found: true; file: File }>

export interface BoardStoreConnector<S> {
    connect(store: S): void
    terminate(): void
}
