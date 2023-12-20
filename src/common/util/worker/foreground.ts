import { WorkerProxyCallID } from "./message"

export interface WorkerProxyMap<P> {
    register(proxy: P): WorkerProxyCallID
    find(id: WorkerProxyCallID): P
    drop(id: WorkerProxyCallID): P
}

export function newWorkerProxyMap<P>(label: string): WorkerProxyMap<P> {
    const map: Map<WorkerProxyCallID, P> = new Map()
    const idFactory = newIdFactory()

    return {
        register(proxy: P): WorkerProxyCallID {
            const id = idFactory()
            map.set(id, proxy)
            return id
        },
        find,
        drop(id: WorkerProxyCallID): P {
            const proxy = find(id)
            map.delete(id)
            return proxy
        },
    }

    function find(id: WorkerProxyCallID): P {
        const proxy = map.get(id)
        if (proxy === undefined) {
            throw new Error(`handler not registered: ${label}`)
        }
        return proxy
    }
}

function newIdFactory(): IdFactory {
    let id = 0
    return () => id++
}

interface IdFactory {
    (): WorkerProxyCallID
}
