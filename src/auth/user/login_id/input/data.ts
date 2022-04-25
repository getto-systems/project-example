import { ValidateTextError } from "../../../../z_lib/ui/validate/data"
import { ConvertBoardFieldResult } from "../../../../z_vendor/getto-application/board/validate_field/data"
import { LoginId } from "../kernel/data"

export type ConvertLoginIdResult = ConvertBoardFieldResult<LoginId, readonly ValidateTextError[]>
