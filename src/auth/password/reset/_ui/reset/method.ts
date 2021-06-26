import { delayedChecker } from "../../../../../z_details/_ui/timer/helper"

import { ResetPasswordInfra } from "./infra"

import { ResetPasswordEvent } from "./event"

import { ConvertBoardResult } from "../../../../../../ui/vendor/getto-application/board/kernel/data"
import { ConvertLocationResult } from "../../../../../z_details/_ui/location/data"
import { ResetToken } from "../../../_ui/data"
import { ResetPasswordFields } from "./data"

export interface ResetPasswordPod {
    (detecter: ResetPasswordDetecter): ResetPasswordMethod
}

export type ResetPasswordDetecter = Detect<ResetToken>

export interface ResetPasswordMethod {
    <S>(
        fields: ConvertBoardResult<ResetPasswordFields>,
        post: Post<ResetPasswordEvent, S>,
    ): Promise<S>
}

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
interface Detect<T> {
    (): ConvertLocationResult<T>
}
