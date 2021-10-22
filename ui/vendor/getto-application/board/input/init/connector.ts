import { BoardValueStore, BoardValueStoreConnector } from "../infra"

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

const initialConnection: BoardValueStoreConnection = { connect: false, hasValue: false }

class Connector implements BoardValueStoreConnector, BoardValueStore {
    conn = initialConnection

    get(): BoardValue {
        if (this.conn.connect) {
            return this.conn.store.get()
        }
        if (this.conn.hasValue) {
            return this.conn.value
        }
        return emptyBoardValue
    }
    set(value: BoardValue): void {
        if (this.conn.connect) {
            this.conn.store.set(value)
        } else {
            this.conn = { connect: false, hasValue: true, value }
        }
    }

    connect(store: BoardValueStore): void {
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

type BoardValueStoreConnection =
    | Readonly<{ connect: false; hasValue: false }>
    | Readonly<{ connect: false; hasValue: true; value: BoardValue }>
    | Readonly<{ connect: true; store: BoardValueStore }>
