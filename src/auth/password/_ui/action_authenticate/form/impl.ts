import { initInputLoginIDAction } from "../../../../login_id/_ui/action_input/core/impl"
import { initInputPasswordAction } from "../../action_input/core/impl"
import { initValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/core/impl"

import { authenticatePasswordFields, AuthenticatePasswordFormAction } from "./action"

import { ConvertBoardResult } from "../../../../../../ui/vendor/getto-application/board/kernel/data"
import { AuthenticatePasswordFields } from "../../authenticate/data"

export function initAuthenticatePasswordFormAction(): AuthenticatePasswordFormAction {
    const loginID = initInputLoginIDAction()
    const password = initInputPasswordAction()

    const validate = initValidateBoardAction({
        fields: authenticatePasswordFields,
        converter,
    })

    loginID.validate.subscriber.subscribe(validate.updateValidateState("loginID"))
    password.validate.subscriber.subscribe(validate.updateValidateState("password"))

    return {
        loginID,
        password,
        validate,
        clear: () => {
            loginID.clear()
            password.clear()
        },
        terminate: () => {
            loginID.terminate()
            password.terminate()
            validate.terminate()
        },
    }

    function converter(): ConvertBoardResult<AuthenticatePasswordFields> {
        const result = {
            loginID: loginID.validate.get(),
            password: password.validate.get(),
        }
        if (!result.loginID.valid || !result.password.valid) {
            return { valid: false }
        }
        return {
            valid: true,
            value: {
                loginID: result.loginID.value,
                password: result.password.value,
            },
        }
    }
}
