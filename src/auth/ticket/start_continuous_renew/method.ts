import { StartContinuousRenewInfra } from "./infra"

import { SaveAuthTicketEvent, StartContinuousRenewEvent } from "./event"

import { AuthTicket, hasExpired } from "../kernel/data"

export interface SaveAuthTicketMethod {
    <S>(auth: AuthTicket, post: Post<SaveAuthTicketEvent, S>): Promise<S>
}

export interface StartContinuousRenewMethod {
    <S>(post: Post<StartContinuousRenewEvent, S>): Promise<S>
}

interface Save {
    (infra: StartContinuousRenewInfra): SaveAuthTicketMethod
}
export const saveAuthTicket: Save = (infra) => async (info, post) => {
    const authnResult = await infra.authn.set(info.authn)
    if (!authnResult.success) {
        return post({ type: "failed-to-save", err: authnResult.err })
    }
    const authzResult = await infra.authz.set(info.authz)
    if (!authzResult.success) {
        return post({ type: "failed-to-save", err: authzResult.err })
    }

    return post({ type: "succeed-to-save" })
}

interface Start {
    (infra: StartContinuousRenewInfra): StartContinuousRenewMethod
}
export const startContinuousRenew: Start = (infra) => (post) => {
    return new Promise((resolve) => {
        const { config } = infra

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
        const { clock, config } = infra

        const result = await infra.authn.get()
        if (!result.success) {
            return { type: "repository-error", continue: false, err: result.err }
        }
        if (!result.found) {
            return clearTicket()
        }

        // 前回の更新時刻が新しければ今回は通信しない
        const time = { now: clock.now(), ...config.authnExpire }
        if (!hasExpired(result.value.authAt, time)) {
            return { type: "authn-not-expired", continue: true }
        }

        const response = await infra.renew()
        if (!response.success) {
            if (response.err.type === "unauthorized") {
                return clearTicket()
            } else {
                return { type: "failed-to-renew", continue: false, err: response.err }
            }
        }

        const authnStoreResult = await infra.authn.set(response.value.authn)
        if (!authnStoreResult.success) {
            return { type: "repository-error", continue: false, err: authnStoreResult.err }
        }

        const authzStoreResult = await infra.authn.set(response.value.authn)
        if (!authzStoreResult.success) {
            return { type: "repository-error", continue: false, err: authzStoreResult.err }
        }

        return { type: "succeed-to-renew", continue: true }

        async function clearTicket(): Promise<StartContinuousRenewEvent> {
            const authnRemoveResult = await infra.authn.remove()
            if (!authnRemoveResult.success) {
                return { type: "repository-error", continue: false, err: authnRemoveResult.err }
            }

            const authzRemoveResult = await infra.authz.remove()
            if (!authzRemoveResult.success) {
                return { type: "repository-error", continue: false, err: authzRemoveResult.err }
            }

            return { type: "required-to-login", continue: false }
        }
    }
}

interface Post<E, S> {
    (event: E): S
}
