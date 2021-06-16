import { RemoteCommonError } from "../../../../../z_details/_ui/remote/data"

export type ResetTokenSendingResult =
    | Readonly<{ done: false; status: ResetTokenSendingStatus }>
    | Readonly<{ done: true; send: false; err: ResetTokenSendingError }>
    | Readonly<{ done: true; send: true }>

export type ResetTokenSendingStatus = Readonly<{ sending: boolean }>

export type ResetTokenSendingError = "failed-to-connect-message-service"

export type CheckResetTokenSendingStatusError = CheckResetTokenSendingStatusRemoteError
export type CheckResetTokenSendingStatusRemoteError =
    | RemoteCommonError
    | Readonly<{ type: "empty-session-id" }>
    | Readonly<{ type: "invalid-reset" }>
    | Readonly<{ type: "already-reset" }>

export type SendResetTokenError = RemoteCommonError
