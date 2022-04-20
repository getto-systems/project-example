import {
    BoardValueStore,
    BoardValueStoreConnector,
    MultipleBoardValueStore,
    FileStore,
    SelectFileResult,
} from "../infra"

export function initBoardValueStoreConnector(): Readonly<{
    connector: BoardValueStoreConnector<BoardValueStore>
    store: BoardValueStore
}> {
    const connector = new Connector()
    return {
        connector,
        store: connector,
    }
}

type Connection = Readonly<{ connect: false }> | Readonly<{ connect: true; store: BoardValueStore }>

const initialConnection: Connection = { connect: false }

class Connector implements BoardValueStoreConnector<BoardValueStore>, BoardValueStore {
    conn = initialConnection
    store = ""

    get(): string {
        if (this.conn.connect) {
            this.store = this.conn.store.get()
        }
        return this.store
    }
    set(value: string): void {
        if (this.conn.connect) {
            this.conn.store.set(value)
        }
        this.store = value
    }

    connect(store: BoardValueStore): void {
        store.set(this.store)
        this.conn = { connect: true, store }
    }
    terminate(): void {
        this.conn = initialConnection
    }
}

export function initMultipleBoardValueStoreConnector(): Readonly<{
    connector: BoardValueStoreConnector<MultipleBoardValueStore>
    store: MultipleBoardValueStore
}> {
    const connector = new MultipleConnector()
    return {
        connector,
        store: connector,
    }
}

type MultipleConnection =
    | Readonly<{ connect: false }>
    | Readonly<{ connect: true; store: MultipleBoardValueStore }>

const initialMultipleConnection: MultipleConnection = { connect: false }

class MultipleConnector
    implements BoardValueStoreConnector<MultipleBoardValueStore>, MultipleBoardValueStore
{
    conn = initialMultipleConnection
    store: readonly string[] = []

    get(): readonly string[] {
        if (this.conn.connect) {
            this.store = this.conn.store.get()
        }
        return this.store
    }
    set(value: readonly string[]): void {
        if (this.conn.connect) {
            this.conn.store.set(value)
        }
        this.store = value
    }

    connect(store: MultipleBoardValueStore): void {
        store.set(this.store)
        this.conn = { connect: true, store }
    }
    terminate(): void {
        this.conn = initialMultipleConnection
    }
}

export function initFileStoreConnector(): Readonly<{
    connector: BoardValueStoreConnector<FileStore>
    store: FileStore
}> {
    const connector = new FileConnector()
    return {
        connector,
        store: connector,
    }
}

type FileConnection = Readonly<{ connect: false }> | Readonly<{ connect: true; store: FileStore }>

const initialFileConnection: FileConnection = { connect: false }

class FileConnector implements BoardValueStoreConnector<FileStore>, FileStore {
    conn = initialFileConnection

    get(): SelectFileResult {
        if (this.conn.connect) {
            return this.conn.store.get()
        }
        return { found: false }
    }

    connect(store: FileStore): void {
        if (this.conn.connect) {
            return
        }
        this.conn = { connect: true, store }
    }
    terminate(): void {
        this.conn = initialFileConnection
    }
}
