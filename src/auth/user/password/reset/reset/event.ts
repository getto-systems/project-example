import { AuthProfile } from "../../../../ticket/kernel/data"
import { ResetPasswordError } from "./data"

export type ResetPasswordEvent =
    | Readonly<{ type: "try-to-reset" }>
    | Readonly<{ type: "take-longtime-to-reset" }>
    | Readonly<{ type: "failed-to-reset"; err: ResetPasswordError }>
    | Readonly<{ type: "succeed-to-reset"; auth: AuthProfile }>
