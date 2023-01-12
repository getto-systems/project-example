import { ValidateTextError } from "../../../../../common/util/validate/data"
import { ValidateBoardFieldResult } from "../../../../../z_vendor/getto-application/board/validate_field/data"
import { TypeAuthUser } from "../../kernel/data"
import { AuthUserTextField } from "./convert"

export type ValidateAuthUserTextResult<K extends AuthUserTextField> = ValidateBoardFieldResult<
    TypeAuthUser<K>,
    readonly ValidateTextError[]
>
