import { MultipleBoardValueStore, MultipleBoardValueStoreConnector } from "../infra"

import { BoardValue } from "../../kernel/data"

export function initMultipleBoardValueStoreConnector(): Readonly<{
    connector: MultipleBoardValueStoreConnector
    store: MultipleBoardValueStore
}> {
    const connector = new Connector()
    return {
        connector,
        store: connector,
    }
}

type Connection =
    | Readonly<{ connect: false; hasValue: false }>
    | Readonly<{ connect: false; hasValue: true; value: BoardValue[] }>
    | Readonly<{ connect: true; store: MultipleBoardValueStore }>

const initialConnection: Connection = { connect: false, hasValue: false }

class Connector implements MultipleBoardValueStoreConnector, MultipleBoardValueStore {
    conn = initialConnection

    get(): BoardValue[] {
        if (this.conn.connect) {
            return this.conn.store.get()
        }
        if (this.conn.hasValue) {
            return this.conn.value
        }
        return []
    }
    set(value: BoardValue[]): void {
        if (this.conn.connect) {
            this.conn.store.set(value)
        } else {
            this.conn = { connect: false, hasValue: true, value }
        }
    }

    connect(store: MultipleBoardValueStore): void {
        if (this.conn.connect) {
            return
        }
        if (this.conn.hasValue) {
            store.set(this.conn.value)
        }
        this.conn = { connect: true, store }
    }
    terminate(): void {
        this.conn = initialConnection
    }
}
