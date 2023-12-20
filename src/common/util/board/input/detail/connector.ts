import {
    BoardStoreConnector,
    SelectFileResult,
    SingleBoardStore,
    MultipleBoardStore,
    FileBoardStore,
} from "../infra"

type SingleConnection =
    | Readonly<{ connect: false }>
    | Readonly<{ connect: true; store: SingleBoardStore }>

const initialSingleConnection: SingleConnection = { connect: false }

export function initSingleBoardStoreConnector(): Readonly<{
    connector: BoardStoreConnector<SingleBoardStore>
    store: SingleBoardStore
}> {
    let conn = initialSingleConnection
    let value = ""

    return {
        connector: {
            connect(store: SingleBoardStore): void {
                store.set(value)
                conn = { connect: true, store }
            },
            terminate(): void {
                conn = initialSingleConnection
            },
        },
        store: {
            get(): string {
                if (conn.connect) {
                    value = conn.store.get()
                }
                return value
            },
            set(newValue: string): void {
                if (conn.connect) {
                    conn.store.set(newValue)
                }
                value = newValue
            },
        },
    }
}

type MultipleConnection =
    | Readonly<{ connect: false }>
    | Readonly<{ connect: true; store: MultipleBoardStore }>

const initialMultipleConnection: MultipleConnection = { connect: false }

export function initMultipleBoardStoreConnector(): Readonly<{
    connector: BoardStoreConnector<MultipleBoardStore>
    store: MultipleBoardStore
}> {
    let conn = initialMultipleConnection
    let value: readonly string[] = []

    return {
        connector: {
            connect(store: MultipleBoardStore): void {
                store.set(value)
                conn = { connect: true, store }
            },
            terminate(): void {
                conn = initialMultipleConnection
            },
        },
        store: {
            get(): readonly string[] {
                if (conn.connect) {
                    value = conn.store.get()
                }
                return value
            },
            set(newValue: readonly string[]): void {
                if (conn.connect) {
                    conn.store.set(newValue)
                }
                value = newValue
            },
        },
    }
}

type FileConnection =
    | Readonly<{ connect: false }>
    | Readonly<{ connect: true; store: FileBoardStore }>

const initialFileConnection: FileConnection = { connect: false }

export function initFileStoreConnector(): Readonly<{
    connector: BoardStoreConnector<FileBoardStore>
    store: FileBoardStore
}> {
    let conn = initialFileConnection

    return {
        connector: {
            connect(store: FileBoardStore): void {
                if (conn.connect) {
                    return
                }
                conn = { connect: true, store }
            },
            terminate(): void {
                conn = initialFileConnection
            },
        },
        store: {
            get(): SelectFileResult {
                if (conn.connect) {
                    return conn.store.get()
                }
                return { found: false }
            },
        },
    }
}
