import { delayedChecker } from "../../../../../z_lib/ui/timer/helper"

import { RequestResetTokenInfra } from "./infra"

import { RequestResetTokenEvent } from "./event"

import { ConvertBoardResult } from "../../../../../../ui/vendor/getto-application/board/kernel/data"
import { RequestResetTokenFields } from "./data"

export interface RequestResetTokenMethod {
    <S>(
        fields: ConvertBoardResult<RequestResetTokenFields>,
        post: Post<RequestResetTokenEvent, S>,
    ): Promise<S>
}

interface RequestToken {
    (infra: RequestResetTokenInfra): RequestResetTokenMethod
}
export const requestResetToken: RequestToken = (infra) => async (fields, post) => {
    if (!fields.valid) {
        return post({ type: "failed-to-request-token", err: { type: "validation-error" } })
    }

    post({ type: "try-to-request-token" })

    const { config } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        infra.requestToken(fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-request-token" }),
    )
    if (!response.success) {
        return post({ type: "failed-to-request-token", err: response.err })
    }

    return post({ type: "succeed-to-request-token" })
}

interface Post<E, S> {
    (event: E): S
}
