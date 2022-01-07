import {
    WorkerProxyCallID,
    WorkerProxyMethod,
    WorkerProxyCallMessage,
    WorkerProxyCallResponse,
} from "./message"

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

// TODO deprecated
export type WorkerProxyErrorMessage<T> = Readonly<{ type: "error"; err: string }> | T
export function handleWorkerProxyError<T>(message: WorkerProxyErrorMessage<T>): void {
    if ("type" in message) {
        if (message.type === "error") {
            throw new Error(message.err)
        }
    }
    throw new Error(JSON.stringify(message))
}

// TODO deprecated
export interface WorkerProxy<M, R> {
    method<N, P, E>(method: N, map: WorkerProxyMessageMapper<N, M, P>): WorkerProxyMethod<N, P, E>
    resolve(response: R): void
}

// TODO deprecated
export interface WorkerProxyMessageMapper<N, M, T> {
    (message: WorkerProxyCallMessage<N, T>): M
}

// TODO deprecated
export abstract class WorkerAbstractProxy<M, R> implements WorkerProxy<M, R> {
    post: PostMessage<M>

    constructor(post: PostMessage<M>) {
        this.post = post
    }

    method<N, T, E>(method: N, map: WorkerProxyMessageMapper<N, M, T>): WorkerProxyMethod<N, T, E> {
        return new ProxyMethod(method, (message) => this.post(map(message)))
    }

    abstract resolve(response: R): void
}
class ProxyMethod<N, M, E> implements WorkerProxyMethod<N, M, E> {
    readonly method: N
    post: PostMessage<WorkerProxyCallMessage<N, M>>

    idGenerator: IDGenerator
    map: Map<number, PostMessage<E>> = new Map()

    constructor(method: N, post: PostMessage<WorkerProxyCallMessage<N, M>>) {
        this.method = method
        this.post = post
        this.idGenerator = idGenerator()
    }

    call<S>(params: M, post: Post<E, S>): Promise<S> {
        return new Promise((resolve) => {
            const id = this.idGenerator()
            this.map.set(id, (event: E) => {
                resolve(post(event))
            })
            this.post({ method: this.method, id, params })
        })
    }
    resolve(response: WorkerProxyCallResponse<N, E>): void {
        const post = this.map.get(response.id)
        if (!post) {
            throw new Error("handler is not set")
        }

        if (!response.done) {
            post(response.event)
        } else {
            this.map.delete(response.id)
        }
    }
}

function idGenerator(): IDGenerator {
    let id = 0
    return () => id++
}

interface IDGenerator {
    (): WorkerProxyCallID
}

interface Post<E, S> {
    (event: E): S
}
interface PostMessage<M> {
    (message: M): void
}
