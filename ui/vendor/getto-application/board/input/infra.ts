import { BoardValue } from "../kernel/data"

export interface BoardValueStore {
    get(): BoardValue
    set(value: BoardValue): void
}

export interface BoardValueStoreConnector {
    connect(store: BoardValueStore): void
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
