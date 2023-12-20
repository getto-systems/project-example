import { ValidateTextError } from "../../../../../common/util/validate/data"
import { ValidateBoardValue } from "../../../../../common/util/board/validate/data"
import { TypeAuthUser } from "../../kernel/data"
import { AuthUserTextFieldName } from "./convert"

export type ValidateAuthUserTextValue<K extends AuthUserTextFieldName> = ValidateBoardValue<
    TypeAuthUser<K>,
    readonly ValidateTextError[]
>
