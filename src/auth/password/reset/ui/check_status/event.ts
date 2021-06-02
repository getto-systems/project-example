import {
    CheckResetTokenSendingStatusError,
    ResetTokenSendingStatus,
    SendResetTokenError,
} from "./data"

export type CheckResetTokenSendingStatusEvent =
    | Readonly<{ type: "try-to-check-status" }>
    | Readonly<{ type: "retry-to-check-status"; status: ResetTokenSendingStatus }>
    | Readonly<{ type: "failed-to-check-status"; err: CheckResetTokenSendingStatusError }>
    | Readonly<{ type: "failed-to-send-token"; err: SendResetTokenError }>
    | Readonly<{ type: "succeed-to-send-token" }>
