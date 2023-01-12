import { ValidateTextError } from "../../../../../../common/util/validate/data"

export type ValidateResetTokenDestinationError =
    | Readonly<{ type: "type"; err: readonly ValidateResetTokenDestinationTypeError[] }>
    | Readonly<{ type: "email"; err: readonly ValidateTextError[] }>

export type ValidateResetTokenDestinationTypeError = Readonly<{ type: "invalid-type" }>
