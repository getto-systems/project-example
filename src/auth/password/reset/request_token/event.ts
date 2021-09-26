import { RequestResetTokenError } from "./data"

export type RequestResetTokenEvent =
    | Readonly<{ type: "try-to-request-token" }>
    | Readonly<{ type: "take-longtime-to-request-token" }>
    | Readonly<{ type: "failed-to-request-token"; err: RequestResetTokenError }>
    | Readonly<{ type: "succeed-to-request-token" }>
