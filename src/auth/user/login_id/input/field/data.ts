import { ValidateTextError } from "../../../../../common/util/validate/data"
import { ValidateBoardValue } from "../../../../../common/util/board/validate/data"
import { LoginId } from "../../kernel/data"

export type ValidateLoginIdValue = ValidateBoardValue<LoginId, readonly ValidateTextError[]>
