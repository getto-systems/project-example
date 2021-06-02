export type WorkerProxySpec<N, P, E> = {
    method: WorkerProxyMethod<N, P, E>
    message: WorkerProxyCallMessage<N, P>
    response: WorkerProxyCallResponse<N, E>
}
export interface WorkerProxyMethod<N, P, E> {
    readonly method: N
    call<S>(params: P, post: Post<E, S>): Promise<S>
    resolve(response: WorkerProxyCallResponse<N, E>): void
}
export type WorkerProxyCallMessage<N, P> = Readonly<{
    method: N
    id: WorkerProxyCallID
    params: P
}>
export type WorkerProxyCallResponse<N, E> =
    | Readonly<{ method: N; id: WorkerProxyCallID; done: false; event: E }>
    | Readonly<{ method: N; id: WorkerProxyCallID; done: true }>

export type WorkerProxyCallID = number

interface Post<E, S> {
    (event: E): S
}
