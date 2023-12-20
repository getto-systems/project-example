import { ValidateTextError } from "../../../../../../../common/util/validate/data"
import {
    ValidateBoardValue,
    SelectBoardValueError,
} from "../../../../../../../common/util/board/validate/data"
import { ResetTokenDestination } from "../../kernel/data"

export type ValidateResetTokenDestinationValue = ValidateBoardValue<
    ResetTokenDestination,
    ValidateResetTokenDestinationError
>

export type ValidateResetTokenDestinationError =
    | Readonly<{ type: "type"; err: SelectBoardValueError }>
    | Readonly<{ type: "email"; err: readonly ValidateTextError[] }>
