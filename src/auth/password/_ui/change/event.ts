import { ChangePasswordError } from "./data"

export type ChangePasswordEvent =
    | Readonly<{ type: "try-to-change-password" }>
    | Readonly<{ type: "take-longtime-to-change-password" }>
    | Readonly<{ type: "failed-to-change-password"; err: ChangePasswordError }>
    | Readonly<{ type: "succeed-to-change-password" }>
