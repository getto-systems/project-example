import { StartContinuousRenewConfig, StartContinuousRenewInfra } from "./infra"

import { StartContinuousRenewEvent } from "./event"

import { hasExpired } from "../kernel/helper"
import { AuthTicket } from "../kernel/data"

type AuthTicketHolder = Readonly<{ hold: false }> | Readonly<{ hold: true; ticket: AuthTicket }>

export async function startContinuousRenew<S>(
    config: StartContinuousRenewConfig,
    infra: StartContinuousRenewInfra,
    holder: AuthTicketHolder,
    post: Post<StartContinuousRenewEvent, S>,
): Promise<S> {
    if (holder.hold) {
        const { ticketRepository } = infra
        const result = await ticketRepository.set(holder.ticket)
        if (!result.success) {
            return post({ type: "repository-error", continue: false, err: result.err })
        }
    }

    return new Promise((resolve) => {
        const timer = setInterval(async () => {
            // 設定された interval ごとに更新
            const result = await continuousRenew()
            const state = post(result)
            if (!result.continue) {
                clearInterval(timer)
                resolve(state)
            }
        }, config.continuousRenewInterval.interval_millisecond)

        post({ type: "succeed-to-start-continuous-renew", continue: true })
    })

    async function continuousRenew(): Promise<StartContinuousRenewEvent> {
        const { clock, ticketRepository, renewRemote } = infra

        const result = await ticketRepository.get()
        if (!result.success) {
            return { type: "repository-error", continue: false, err: result.err }
        }
        if (!result.found) {
            return clearProfile()
        }

        // 前回の更新時刻が新しければ今回は通信しない
        const time = { now: clock.now(), ...config.ticketExpire }
        if (!hasExpired(result.value, time)) {
            return { type: "ticket-not-expired", continue: true }
        }

        const response = await renewRemote()
        if (!response.success) {
            if (response.err.type === "unauthorized") {
                return clearProfile()
            } else {
                return { type: "failed-to-renew", continue: false, err: response.err }
            }
        }

        const saveProfileResult = await ticketRepository.set(response.value)
        if (!saveProfileResult.success) {
            return { type: "repository-error", continue: false, err: saveProfileResult.err }
        }

        return { type: "succeed-to-renew", continue: true }

        async function clearProfile(): Promise<StartContinuousRenewEvent> {
            const removeProfileResult = await ticketRepository.remove()
            if (!removeProfileResult.success) {
                return { type: "repository-error", continue: false, err: removeProfileResult.err }
            }

            return { type: "required-to-login", continue: false }
        }
    }
}

interface Post<E, S> {
    (event: E): S
}
