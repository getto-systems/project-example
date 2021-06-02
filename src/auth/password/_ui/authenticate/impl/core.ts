import { delayedChecker } from "../../../../../../ui/vendor/getto-application/infra/timer/helper"

import { AuthenticatePasswordInfra } from "../infra"

import { AuthenticatePasswordMethod } from "../method"
import { authRemoteConverter } from "../../../../auth_ticket/_ui/kernel/converter"

interface Authenticate {
    (infra: AuthenticatePasswordInfra): AuthenticatePasswordMethod
}
export const authenticatePassword: Authenticate = (infra) => async (fields, post) => {
    if (!fields.valid) {
        return post({ type: "failed-to-login", err: { type: "validation-error" } })
    }

    post({ type: "try-to-login" })

    const { clock, config } = infra
    const authenticate = infra.authenticate(authRemoteConverter(clock))

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(authenticate(fields.value), config.takeLongtimeThreshold, () =>
        post({ type: "take-longtime-to-login" }),
    )
    if (!response.success) {
        return post({ type: "failed-to-login", err: response.err })
    }

    return post({ type: "succeed-to-login", auth: response.value })
}
