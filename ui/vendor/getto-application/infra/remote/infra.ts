export type RemoteOutsideFeature = Readonly<{
    webCrypto: Crypto
}>
export type RemoteFeature = Readonly<{
    serverURL: string
    nonce: RemoteNonceGenerator
}>

export interface RemoteNonceGenerator {
    (): RemoteNonce
}
export type RemoteNonce = string

export type RemoteTypes<M, V, R, E> = {
    pod: RemotePod<M, V, R, E>
    remote: Remote<M, V, E>
    result: RemoteResult<R, E>
    simulator: RemoteSimulator<M, R, E>
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
