import {
    BoardValueStore,
    BoardValueStoreConnector,
    MultipleBoardValueStore,
    FileStore,
    SelectFileResult,
} from "../infra"

type Connection = Readonly<{ connect: false }> | Readonly<{ connect: true; store: BoardValueStore }>

const initialConnection: Connection = { connect: false }

export function initBoardValueStoreConnector(): Readonly<{
    connector: BoardValueStoreConnector<BoardValueStore>
    store: BoardValueStore
}> {
    let conn = initialConnection
    let value = ""

    return {
        connector: {
            connect(store: BoardValueStore): void {
                store.set(value)
                conn = { connect: true, store }
            },
            terminate(): void {
                conn = initialConnection
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
    | Readonly<{ connect: true; store: MultipleBoardValueStore }>

const initialMultipleConnection: MultipleConnection = { connect: false }

export function initMultipleBoardValueStoreConnector(): Readonly<{
    connector: BoardValueStoreConnector<MultipleBoardValueStore>
    store: MultipleBoardValueStore
}> {
    let conn = initialMultipleConnection
    let value: readonly string[] = []

    return {
        connector: {
            connect(store: MultipleBoardValueStore): void {
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

type FileConnection = Readonly<{ connect: false }> | Readonly<{ connect: true; store: FileStore }>

const initialFileConnection: FileConnection = { connect: false }

export function initFileStoreConnector(): Readonly<{
    connector: BoardValueStoreConnector<FileStore>
    store: FileStore
}> {
    let conn = initialFileConnection

    return {
        connector: {
            connect(store: FileStore): void {
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
