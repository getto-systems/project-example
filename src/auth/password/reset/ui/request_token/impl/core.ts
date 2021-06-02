import { delayedChecker } from "../../../../../../../ui/vendor/getto-application/infra/timer/helper"

import { RequestResetTokenInfra } from "../infra"

import { RequestResetTokenMethod } from "../method"

import { resetSessionIDRemoteConverter } from "../../converter"

interface RequestToken {
    (infra: RequestResetTokenInfra): RequestResetTokenMethod
}
export const requestResetToken: RequestToken = (infra) => async (fields, post) => {
    if (!fields.valid) {
        return post({ type: "failed-to-request-token", err: { type: "validation-error" } })
    }

    post({ type: "try-to-request-token" })

    const { config } = infra
    const requestToken = infra.requestToken(resetSessionIDRemoteConverter)

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        requestToken(fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-request-token" }),
    )
    if (!response.success) {
        return post({ type: "failed-to-request-token", err: response.err })
    }

    return post({ type: "succeed-to-request-token", sessionID: response.value })
}
