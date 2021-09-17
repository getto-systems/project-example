import { delayedChecker } from "../../../../z_details/_ui/timer/helper"

import { ChangePasswordInfra } from "./infra"

import { ChangePasswordEvent } from "./event"

import { ConvertBoardResult } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { ChangePasswordFields } from "./data"

export interface ChangePasswordMethod {
    <S>(
        fields: ConvertBoardResult<ChangePasswordFields>,
        post: Post<ChangePasswordEvent, S>,
    ): Promise<S>
}

interface Change {
    (infra: ChangePasswordInfra): ChangePasswordMethod
}
export const changePassword: Change = (infra) => async (fields, post) => {
    if (!fields.valid) {
        return post({ type: "failed-to-change-password", err: { type: "validation-error" } })
    }

    post({ type: "try-to-change-password" })

    const { config } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        infra.change(fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-change-password" }),
    )
    if (!response.success) {
        return post({ type: "failed-to-change-password", err: response.err })
    }

    return post({ type: "succeed-to-change-password" })
}

interface Post<E, S> {
    (event: E): S
}
