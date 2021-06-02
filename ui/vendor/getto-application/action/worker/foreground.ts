import {
    WorkerProxyCallID,
    WorkerProxyMethod,
    WorkerProxyCallMessage,
    WorkerProxyCallResponse,
} from "./message"

export interface WorkerProxy<M, R> {
    method<N, P, E>(method: N, map: WorkerProxyMessageMapper<N, M, P>): WorkerProxyMethod<N, P, E>
    resolve(response: R): void
}

export interface WorkerProxyMessageMapper<N, M, T> {
    (message: WorkerProxyCallMessage<N, T>): M
}

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
