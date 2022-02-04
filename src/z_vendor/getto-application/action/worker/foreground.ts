import { WorkerProxyCallID } from "./message"

export interface WorkerProxyMap<P> {
    register(proxy: P): WorkerProxyCallID
    find(id: WorkerProxyCallID): P
    drop(id: WorkerProxyCallID): P
}

export function newWorkerProxyMap<P>(label: string): WorkerProxyMap<P> {
    return new ProxyMap(label)
}

class ProxyMap<P> implements WorkerProxyMap<P> {
    label: string

    map: Map<WorkerProxyCallID, P> = new Map()
    idGenerator = idGenerator()

    constructor(label: string) {
        this.label = label
    }

    register(proxy: P): WorkerProxyCallID {
        const id = this.idGenerator()
        this.map.set(id, proxy)
        return id
    }
    find(id: WorkerProxyCallID): P {
        const proxy = this.map.get(id)
        if (proxy === undefined) {
            throw new Error(`handler not registered: ${this.label}`)
        }
        return proxy
    }
    drop(id: WorkerProxyCallID): P {
        const proxy = this.find(id)
        this.map.delete(id)
        return proxy
    }
}

function idGenerator(): IDGenerator {
    let id = 0
    return () => id++
}

interface IDGenerator {
    (): WorkerProxyCallID
}
