export type RemoteCommonError =
    | Readonly<{ type: "unauthorized" }>
    | RemoteServerError
    | RemoteInfraError

export type RemoteServerError = Readonly<{ type: "server-error" }>
export type RemoteInfraError = Readonly<{ type: "infra-error"; err: string }>
