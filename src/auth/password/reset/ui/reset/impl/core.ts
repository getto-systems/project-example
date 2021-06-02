import { delayedChecker } from "../../../../../../../ui/vendor/getto-application/infra/timer/helper"

import { authRemoteConverter } from "../../../../../auth_ticket/_ui/kernel/converter"
import { resetTokenLocationConverter } from "../../converter"

import { ResetPasswordInfra } from "../infra"

import { ResetPasswordLocationDetectMethod, ResetPasswordPod } from "../method"

import { SignNav, signNavKey } from "../../../../../_ui/common/nav/data"

export const detectResetToken: ResetPasswordLocationDetectMethod = (currentURL) =>
    resetTokenLocationConverter(currentURL.searchParams.get(signNavKey(SignNav.passwordResetToken)))

interface Reset {
    (infra: ResetPasswordInfra): ResetPasswordPod
}
export const resetPassword: Reset = (infra) => (detecter) => async (fields, post) => {
    if (!fields.valid) {
        return post({ type: "failed-to-reset", err: { type: "validation-error" } })
    }

    const resetToken = detecter()
    if (!resetToken.valid) {
        return post({ type: "failed-to-reset", err: { type: "empty-reset-token" } })
    }

    post({ type: "try-to-reset" })

    const { clock, config } = infra
    const reset = infra.reset(authRemoteConverter(clock))

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        reset({ resetToken: resetToken.value, fields: fields.value }),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-reset" }),
    )
    if (!response.success) {
        return post({ type: "failed-to-reset", err: response.err })
    }

    return post({ type: "succeed-to-reset", auth: response.value })
}
