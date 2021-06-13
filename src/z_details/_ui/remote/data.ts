export type RemoteCommonError =
    | Readonly<{ type: "unauthorized" }>
    | Readonly<{ type: "invalid-nonce" }>
    | Readonly<{ type: "server-error" }>
    | Readonly<{ type: "infra-error"; err: string }>
