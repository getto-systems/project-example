import { ValidateTextError } from "../../../../z_lib/ui/validate/data"
import { ValidateBoardFieldResult } from "../../../../z_vendor/getto-application/board/validate_field/data"

export type Password = string & { Password: never }

export type ValidatePasswordResult = ValidateBoardFieldResult<
    Password,
    readonly ValidateTextError[]
>
