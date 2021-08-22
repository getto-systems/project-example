import { initInputLoginIDAction } from "../../../../../login_id/_ui/action_input/core/impl"
import { initValidateBoardAction } from "../../../../../../../ui/vendor/getto-application/board/action_validate_board/init"

import { RequestResetTokenFormAction, requestResetTokenFields } from "./action"

import { ConvertBoardResult } from "../../../../../../../ui/vendor/getto-application/board/kernel/data"
import { RequestResetTokenFields } from "../../request_token/data"

export function initRequestResetTokenFormAction(): RequestResetTokenFormAction {
    const loginID = initInputLoginIDAction()
    const validate = initValidateBoardAction({
        fields: requestResetTokenFields,
        converter,
    })

    loginID.validate.subscriber.subscribe(validate.updateValidateState("loginID"))

    return {
        loginID,
        validate,
        clear: () => loginID.clear(),
        terminate: () => loginID.terminate(),
    }

    function converter(): ConvertBoardResult<RequestResetTokenFields> {
        const loginIDResult = loginID.validate.get()
        if (!loginIDResult.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                loginID: loginIDResult.value,
            },
        }
    }
}
