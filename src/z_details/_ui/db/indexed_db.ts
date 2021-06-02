import {
    DBErrorResult,
    FetchDBResult,
    FromDBConverter,
    StoreDBResult,
    ToDBConverter,
} from "./infra"

export function initIndexedDB(webDB: IDBFactory, config: IndexedDBConfig): IndexedDB {
    return new DB(webDB, config)
}

// objectStore(`name`, { keyPath: "key" }) に { key: string, value: string } という値を保存する
export interface IndexedDB {
    get<T>(store: IndexedDBTarget, converter: FromDBConverter<T>): Promise<FetchDBResult<T>>
    set<T>(store: IndexedDBTarget, converter: ToDBConverter<T>, value: T): Promise<StoreDBResult>
    remove(store: IndexedDBTarget): Promise<StoreDBResult>
}

// データベース名と作成する objectStore を指定する
export type IndexedDBConfig = Readonly<{
    database: string
    stores: string[]
}>

export type IndexedDBTarget = Readonly<{
    store: string
    key: string
}>

// 構造を変えるときは migration を追加することで対応
const MIGRATIONS: Migration[] = [
    (db, stores) => {
        stores.forEach((store) => {
            db.createObjectStore(store, { keyPath: "key" })
        })
    },
]

interface Migration {
    (db: IDBDatabase, stores: string[]): void
}

class DB implements IndexedDB {
    webDB: IDBFactory
    config: IndexedDBConfig

    constructor(webDB: IDBFactory, config: IndexedDBConfig) {
        this.webDB = webDB
        this.config = config
    }

    get<T>(target: IndexedDBTarget, converter: FromDBConverter<T>): Promise<FetchDBResult<T>> {
        return new Promise((resolve) => {
            this.open(resolve, (db) => {
                try {
                    const tx = db.transaction(target.store)
                    tx.oncomplete = () => db.close()

                    const request = tx.objectStore(target.store).get(target.key)
                    request.onsuccess = (e: Event) => {
                        if (!e.target || !(e.target instanceof IDBRequest)) {
                            resolve(dbError("invalid get result"))
                            return
                        }
                        if (!e.target.result) {
                            resolve({ success: true, found: false })
                            return
                        }

                        try {
                            // e.target.result は any のため、実行時エラーを覚悟する
                            // ブラウザのオブジェクトストレージの内容が any なのは本質的で避けられない
                            resolve({
                                success: true,
                                found: true,
                                value: converter(e.target.result.value),
                            })
                        } catch (err) {
                            resolve(dbError(`${err}`))
                        }
                    }
                    request.onerror = () => {
                        resolve(dbError("failed to get"))
                    }
                } catch (err) {
                    resolve(dbError(`${err}`))
                }
            })
        })
    }

    set<T>(target: IndexedDBTarget, converter: ToDBConverter<T>, value: T): Promise<StoreDBResult> {
        return new Promise((resolve) => {
            this.open(resolve, (db) => {
                try {
                    const tx = db.transaction(target.store, "readwrite")
                    tx.oncomplete = () => db.close()

                    const request = tx
                        .objectStore(target.store)
                        .put({ key: target.key, value: converter(value) })
                    request.onsuccess = () => {
                        resolve({ success: true })
                    }
                    request.onerror = () => {
                        resolve(dbError("failed to put"))
                    }
                } catch (err) {
                    resolve(dbError(`${err}`))
                }
            })
        })
    }

    remove(target: IndexedDBTarget): Promise<StoreDBResult> {
        return new Promise((resolve) => {
            this.open(resolve, (db) => {
                try {
                    const tx = db.transaction(target.store, "readwrite")
                    tx.oncomplete = () => db.close()

                    const request = tx.objectStore(target.store).delete(target.key)
                    request.onsuccess = () => {
                        resolve({ success: true })
                    }
                    request.onerror = () => {
                        resolve(dbError("failed to remove"))
                    }
                } catch (err) {
                    resolve(dbError(`${err}`))
                }
            })
        })
    }

    open(error: Post<DBErrorResult>, success: Post<IDBDatabase>): void {
        const request = this.webDB.open(this.config.database, MIGRATIONS.length)
        request.onupgradeneeded = upgrade(this.config.stores, MIGRATIONS)
        request.onerror = () => {
            error(dbError("failed to open db"))
        }
        request.onsuccess = (e: Event) => {
            if (!e.target || !(e.target instanceof IDBOpenDBRequest)) {
                error(dbError("invalid open db result"))
                return
            }
            success(e.target.result)
        }

        function upgrade(
            stores: string[],
            migrations: Migration[],
        ): { (e: IDBVersionChangeEvent): void } {
            return (e) => {
                if (!e.target || !(e.target instanceof IDBOpenDBRequest)) {
                    return
                }
                const db = e.target.result
                migrations
                    .slice(e.oldVersion, e.newVersion === null ? undefined : e.newVersion)
                    .forEach((migration) => migration(db, stores))
            }
        }
    }
}

function dbError(err: string): DBErrorResult {
    return { success: false, err: { type: "infra-error", err } }
}

interface Post<T> {
    (result: T): void
}
