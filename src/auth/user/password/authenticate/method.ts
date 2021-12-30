import { delayedChecker } from "../../../../z_lib/ui/timer/helper"

import { AuthenticatePasswordInfra } from "./infra"

import { AuthenticatePasswordEvent } from "./event"

import { ConvertBoardResult } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { AuthenticatePasswordFields } from "./data"

export interface AuthenticatePasswordMethod {
    <S>(
        fields: ConvertBoardResult<AuthenticatePasswordFields>,
        post: Post<AuthenticatePasswordEvent, S>,
    ): Promise<S>
}

interface Authenticate {
    (infra: AuthenticatePasswordInfra): AuthenticatePasswordMethod
}
export const authenticatePassword: Authenticate = (infra) => async (fields, post) => {
    if (!fields.valid) {
        return post({ type: "failed-to-login", err: { type: "validation-error" } })
    }

    post({ type: "try-to-login" })

    const { config, authenticateRemote } = infra

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        authenticateRemote(fields.value),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-login" }),
    )
    if (!response.success) {
        return post({ type: "failed-to-login", err: response.err })
    }

    return post({ type: "succeed-to-login", auth: response.value })
}

interface Post<E, S> {
    (event: E): S
}
