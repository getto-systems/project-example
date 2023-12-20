export type ConnectState =
    | Readonly<{ isConnecting: false }>
    | Readonly<{ isConnecting: true; hasTakenLongtime: boolean }>

export type SuccessState = Readonly<{ isSuccess: boolean }>
