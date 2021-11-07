import { delayedChecker } from "../../../../../z_lib/ui/timer/helper"

import { ResetPasswordInfra } from "./infra"

import { ResetPasswordEvent } from "./event"

import { ConvertBoardResult } from "../../../../../../ui/vendor/getto-application/board/kernel/data"
import { ConvertLocationResult } from "../../../../../z_lib/ui/location/data"
import { ResetToken } from "../../input/data"
import { ResetPasswordFields } from "./data"

export interface ResetPasswordMethod {
    <S>(
        resetToken: ConvertLocationResult<ResetToken>,
        fields: ConvertBoardResult<ResetPasswordFields>,
        post: Post<ResetPasswordEvent, S>,
    ): Promise<S>
}

interface Reset {
    (infra: ResetPasswordInfra): ResetPasswordMethod
}
export const resetPassword: Reset = (infra) => async (resetToken, fields, post) => {
    if (!fields.valid) {
        return post({ type: "failed-to-reset", err: { type: "validation-error" } })
    }

    if (!resetToken.valid) {
        return post({ type: "failed-to-reset", err: { type: "empty-reset-token" } })
    }

    post({ type: "try-to-reset" })

    const { config } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        infra.reset(resetToken.value, fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-reset" }),
    )
    if (!response.success) {
        return post({ type: "failed-to-reset", err: response.err })
    }

    return post({ type: "succeed-to-reset", auth: response.value })
}

interface Post<E, S> {
    (event: E): S
}
