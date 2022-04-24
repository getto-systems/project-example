import { ValidateTextError } from "../../../../../z_lib/ui/validate/data"
import { ConvertBoardFieldResult } from "../../../../../z_vendor/getto-application/board/validate_field/data"
import { AuthUserMemo } from "../../kernel/data"

export type ConvertAuthUserMemoResult = ConvertBoardFieldResult<
    AuthUserMemo,
    readonly ValidateTextError[]
>
