import { AuthTicket } from "../../ticket/kernel/data"
import { AuthenticatePasswordError } from "./data"

export type AuthenticatePasswordEvent =
    | Readonly<{ type: "try-to-login" }>
    | Readonly<{ type: "take-longtime-to-login" }>
    | Readonly<{ type: "failed-to-login"; err: AuthenticatePasswordError }>
    | Readonly<{ type: "succeed-to-login"; auth: AuthTicket }>
