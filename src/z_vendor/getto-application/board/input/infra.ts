export interface BoardValueStore {
    get(): string
    set(value: string): void
}
export interface MultipleBoardValueStore {
    get(): readonly string[]
    set(value: readonly string[]): void
}
export interface FileStore {
    get(): SelectFileResult
}

export type SelectFileResult = Readonly<{ found: false }> | Readonly<{ found: true; file: File }>

export interface BoardValueStoreConnector<S> {
    connect(store: S): void
    terminate(): void
}

export interface InputBoardEventPublisher {
    post(): void
}
export interface InputBoardEventSubscriber {
    subscribe(handler: InputBoardEventHandler): void
    terminate(): void
}
export interface InputBoardEventHandler {
    (): void
}
