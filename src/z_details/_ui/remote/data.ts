export type RemoteResult<V, E> = Readonly<{ success: true; value: V }> | RemoteErrorResult<E>
export type RemoteErrorResult<E> = Readonly<{ success: false; err: E }>

export type RemoteCommonError =
    | Readonly<{ type: "unauthorized" }>
    | Readonly<{ type: "invalid-nonce" }>
    | RemoteServerError
    | RemoteInfraError

export type RemoteServerError = Readonly<{ type: "server-error" }>
export type RemoteInfraError = Readonly<{ type: "infra-error"; err: string }>
