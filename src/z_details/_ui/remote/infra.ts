export type RemoteFetchOptions = Readonly<{
    url: string
    options: Readonly<{
        method: RemoteFetchMethod
        credentials: "include"
        headers: RemoteHeader[]
    }>
}>

export type RemoteFetchMethod = "GET" | "POST" | "PATCH" | "PUT" | "DELETE"
export type RemoteHeader = [string, string]

export type RemoteNonce = string

export type RemoteTypes<M, V, R, E> = {
    pod: RemotePod<M, V, R, E>
    remote: Remote<M, V, E>
    result: RemoteResult<R, E>
}

export interface RemotePod<M, V, R, E> {
    (converter: RemoteConverter<V, R>): Remote<M, V, E>
}
export interface Remote<M, V, E> {
    (message: M): Promise<RemoteResult<V, E>>
}
export interface RemoteSimulator<M, V, E> {
    (message: M): RemoteResult<V, E>
}

export type RemoteResult<V, E> =
    | Readonly<{ success: true; value: V }>
    | Readonly<{ success: false; err: E }>

export interface RemoteConverter<V, R> {
    (raw: R): V
}
