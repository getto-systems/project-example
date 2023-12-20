import { ValidateTextError } from "../../../../../common/util/validate/data"
import { ValidateBoardValue } from "../../../../../common/util/board/validate/data"

export type Password = string & { Password: never }

export type ValidatePasswordValue = ValidateBoardValue<Password, readonly ValidateTextError[]>
