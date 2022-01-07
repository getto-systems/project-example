// TODO deprecated
export type WorkerProxySpec<N, P, E> = {
    method: WorkerProxyMethod<N, P, E>
    message: WorkerProxyCallMessage<N, P>
    response: WorkerProxyCallResponse<N, E>
}
// TODO deprecated
export interface WorkerProxyMethod<N, P, E> {
    readonly method: N
    call<S>(params: P, post: Post<E, S>): Promise<S>
    resolve(response: WorkerProxyCallResponse<N, E>): void
}
// TODO deprecated
export type WorkerProxyCallMessage<N, P> = Readonly<{
    method: N
    id: WorkerProxyCallID
    params: P
}>
// TODO deprecated
export type WorkerProxyCallResponse<N, E> =
    | Readonly<{ method: N; id: WorkerProxyCallID; done: false; event: E }>
    | Readonly<{ method: N; id: WorkerProxyCallID; done: true }>

export type WorkerProxyCallID = number

export type WorkerProxyMessage<N, T> = Readonly<{
    id: WorkerProxyCallID
    name: N
    params: Readonly<T>
}>
export type WorkerProxyResponse<N, T> = Readonly<{ id: WorkerProxyCallID; name: N; data: T }>

interface Post<E, S> {
    (event: E): S
}
