import { delayedChecker } from "../../../z_lib/ui/timer/helper"

import { CheckAuthTicketInfra } from "./infra"

import { RenewAuthTicketEvent, CheckAuthTicketEvent } from "./event"

import { hasExpired } from "../kernel/data"

export interface CheckAuthTicketMethod {
    <S>(post: Post<CheckAuthTicketEvent, S>): Promise<S>
}

interface Check {
    (infra: CheckAuthTicketInfra): CheckAuthTicketMethod
}
export const checkAuthTicket: Check = (infra) => async (post) => {
    const { clock, config, profileRepository: profile_repository } = infra

    const findResult = await profile_repository.get()
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
    if (!hasExpired(findResult.value, time)) {
        return post({ type: "try-to-instant-load" })
    }

    return renewTicket(infra, post)
}

export interface RenewAuthTicketMethod {
    <S>(post: Post<RenewAuthTicketEvent, S>): Promise<S>
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
    const { config, profileRepository, renewRemote } = infra

    post({ type: "try-to-renew" })

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(renewRemote(), config.takeLongtimeThreshold, () =>
        post({ type: "take-longtime-to-renew" }),
    )
    if (!response.success) {
        if (response.err.type === "unauthorized") {
            const removeResult = await profileRepository.remove()
            if (!removeResult.success) {
                return post({ type: "repository-error", err: removeResult.err })
            }
            return post({ type: "required-to-login" })
        }
        return post({ type: "failed-to-renew", err: response.err })
    }

    const profileResult = await profileRepository.set(response.value)
    if (!profileResult.success) {
        return post({ type: "repository-error", err: profileResult.err })
    }

    return post({ type: "succeed-to-renew", auth: response.value })
}

interface Post<E, S> {
    (event: E): S
}
