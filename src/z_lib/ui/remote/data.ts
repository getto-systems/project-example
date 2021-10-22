export type RemoteCommonError =
    | Readonly<{ type: "unauthorized" }>
    | Readonly<{ type: "invalid-nonce" }>
    | RemoteServerError
    | RemoteInfraError

export type RemoteServerError = Readonly<{ type: "server-error" }>
export type RemoteInfraError = Readonly<{ type: "infra-error"; err: string }>

export type SearchPage = Readonly<{
    offset: number
    limit: number
    all: number
}>
