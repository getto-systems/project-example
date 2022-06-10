import { ValidateTextError } from "../../../../../../z_lib/ui/validate/data"
import { ValidateBoardFieldResult } from "../../../../../../z_vendor/getto-application/board/validate_field/data"

export type ResetTokenDestination =
    | Readonly<{ type: "none" }>
    | Readonly<{ type: "email"; email: ResetTokenDestinationEmail }>
export type ResetTokenDestinationEmail = string & { ResetTokenDestinationEmail: never }

export type ValidateResetTokenDestinationError =
    | Readonly<{ type: "type"; err: readonly ValidateResetTokenDestinationTypeError[] }>
    | Readonly<{ type: "email"; err: readonly ValidateTextError[] }>

export type ValidateResetTokenDestinationTypeError = Readonly<{ type: "invalid-type" }>

export type ValidateResetTokenDestinationResult = ValidateBoardFieldResult<
    ResetTokenDestination,
    ValidateResetTokenDestinationError
>
