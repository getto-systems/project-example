import {
    BoardValueStore,
    BoardValueStoreConnector,
    MultipleBoardValueStore,
    MultipleBoardValueStoreConnector,
    FileStore,
    FileStoreConnector,
    SelectFileResult,
} from "../infra"

import { BoardValue, emptyBoardValue } from "../../kernel/data"

export function initBoardValueStoreConnector(): Readonly<{
    connector: BoardValueStoreConnector
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

class Connector implements BoardValueStoreConnector, BoardValueStore {
    conn = initialConnection
    store = emptyBoardValue

    get(): BoardValue {
        if (this.conn.connect) {
            this.store = this.conn.store.get()
        }
        return this.store
    }
    set(value: BoardValue): void {
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
    connector: MultipleBoardValueStoreConnector
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

class MultipleConnector implements MultipleBoardValueStoreConnector, MultipleBoardValueStore {
    conn = initialMultipleConnection
    store: readonly BoardValue[] = []

    get(): readonly BoardValue[] {
        if (this.conn.connect) {
            this.store = this.conn.store.get()
        }
        return this.store
    }
    set(value: readonly BoardValue[]): void {
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
    connector: FileStoreConnector
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

class FileConnector implements FileStoreConnector, FileStore {
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
