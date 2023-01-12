import { ValidateTextError } from "../../../../common/util/validate/data"
import { ValidateBoardFieldResult } from "../../../../z_vendor/getto-application/board/validate_field/data"
import { LoginId } from "../kernel/data"

export type ConvertLoginIdResult = ValidateBoardFieldResult<LoginId, readonly ValidateTextError[]>
