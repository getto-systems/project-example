import { BoardValue } from "../kernel/data"

export interface BoardValueStore {
    get(): BoardValue
    set(value: BoardValue): void
}

export interface BoardValueStoreConnector {
    connect(store: BoardValueStore): void
    terminate(): void
}

export interface MultipleBoardValueStore {
    get(): readonly BoardValue[]
    set(value: readonly BoardValue[]): void
}

export interface MultipleBoardValueStoreConnector {
    connect(store: MultipleBoardValueStore): void
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