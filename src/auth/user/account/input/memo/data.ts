import { ValidateTextError } from "../../../../../z_lib/ui/validate/data"
import { ValidateBoardFieldResult } from "../../../../../z_vendor/getto-application/board/validate_field/data"
import { AuthUserMemo } from "../../kernel/data"

export type ConvertAuthUserMemoResult = ValidateBoardFieldResult<
    AuthUserMemo,
    readonly ValidateTextError[]
>
