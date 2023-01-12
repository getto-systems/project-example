import { AuthTicketRepository } from "../kernel/infra"
import { CheckAuthTicketRemote } from "./infra"
import { Clock } from "../../../common/util/clock/infra"
import { ExpireTime, IntervalTime } from "../../../common/util/config/infra"

import { hasExpired } from "../kernel/helper"

import { AuthTicket } from "../kernel/data"
import { RemoteCommonError } from "../../../common/util/remote/data"
import { RepositoryError } from "../../../common/util/repository/data"

export type StartContinuousRenewEvent =
    | Readonly<{ type: "succeed-to-start-continuous-renew"; continue: true }>
    | Readonly<{ type: "ticket-not-expired"; continue: true }>
    | Readonly<{ type: "succeed-to-renew"; continue: true }>
    | Readonly<{ type: "required-to-login"; continue: false }>
    | Readonly<{ type: "failed-to-renew"; continue: false; err: RemoteCommonError }>
    | Readonly<{ type: "repository-error"; continue: false; err: RepositoryError }>

export type StartContinuousRenewMaterial = Readonly<{
    infra: StartContinuousRenewInfra
    config: StartContinuousRenewConfig
}>

export type StartContinuousRenewInfra = Readonly<{
    ticketRepository: AuthTicketRepository
    renewRemote: CheckAuthTicketRemote
    clock: Clock
}>

export type StartContinuousRenewConfig = Readonly<{
    continuousRenewInterval: IntervalTime
    ticketExpire: ExpireTime
}>

type AuthTicketHolder =
    | Readonly<{ hasTicket: false }>
    | Readonly<{ hasTicket: true; ticket: AuthTicket }>

export async function startContinuousRenew<S>(
    { infra, config }: StartContinuousRenewMaterial,
    holder: AuthTicketHolder,
    post: Post<StartContinuousRenewEvent, S>,
): Promise<S> {
    if (holder.hasTicket) {
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
