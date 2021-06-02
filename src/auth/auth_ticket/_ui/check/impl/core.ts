import { delayedChecker } from "../../../../../../ui/vendor/getto-application/infra/timer/helper"

import { CheckAuthTicketInfra } from "../infra"

import { RenewAuthTicketMethod, CheckAuthTicketMethod } from "../method"

import { RenewAuthTicketEvent } from "../event"

import { hasExpired } from "../../kernel/data"
import { authnRepositoryConverter, authRemoteConverter } from "../../kernel/converter"
import { authzRepositoryConverter } from "../../kernel/converter"

interface Check {
    (infra: CheckAuthTicketInfra): CheckAuthTicketMethod
}
export const checkAuthTicket: Check = (infra) => async (post) => {
    const { clock, config } = infra
    const authn = infra.authn(authnRepositoryConverter)

    const findResult = await authn.get()
    if (!findResult.success) {
        return post({ type: "repository-error", err: findResult.err })
    }
    if (!findResult.found) {
        return post({ type: "required-to-login" })
    }

    const time = {
        now: clock.now(),
        expire_millisecond: config.instantLoadExpire.expire_millisecond,
    }
    if (!hasExpired(findResult.value.authAt, time)) {
        return post({ type: "try-to-instant-load" })
    }

    return renewTicket(infra, post)
}

interface RenewAuthTicket {
    (infra: CheckAuthTicketInfra): RenewAuthTicketMethod
}
export const renewAuthTicket: RenewAuthTicket = (infra) => async (post) => {
    return renewTicket(infra, post)
}

async function renewTicket<S>(
    infra: CheckAuthTicketInfra,
    post: Post<RenewAuthTicketEvent, S>,
): Promise<S> {
    const { clock, config } = infra
    const authn = infra.authn(authnRepositoryConverter)
    const authz = infra.authz(authzRepositoryConverter)
    const renew = infra.renew(authRemoteConverter(clock))

    post({ type: "try-to-renew" })

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(
        renew({ type: "always" }),
        config.takeLongtimeThreshold,
        () => post({ type: "take-longtime-to-renew" }),
    )
    if (!response.success) {
        if (response.err.type === "unauthorized") {
            const removeResult = await authn.remove()
            if (!removeResult.success) {
                return post({ type: "repository-error", err: removeResult.err })
            }
            return post({ type: "required-to-login" })
        }
        return post({ type: "failed-to-renew", err: response.err })
    }

    const authnResult = await authn.set(response.value.authn)
    if (!authnResult.success) {
        return post({ type: "repository-error", err: authnResult.err })
    }

    const authzResult = await authz.set(response.value.authz)
    if (!authzResult.success) {
        return post({ type: "repository-error", err: authzResult.err })
    }

    return post({ type: "succeed-to-renew", auth: response.value })
}

interface Post<E, S> {
    (event: E): S
}
