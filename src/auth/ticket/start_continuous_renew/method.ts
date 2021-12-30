import { StartContinuousRenewInfra } from "./infra"

import { SaveAuthProfileEvent, StartContinuousRenewEvent } from "./event"

import { AuthProfile, hasExpired } from "../kernel/data"

export interface SaveAuthProfileMethod {
    <S>(profile: AuthProfile, post: Post<SaveAuthProfileEvent, S>): Promise<S>
}

export interface StartContinuousRenewMethod {
    <S>(post: Post<StartContinuousRenewEvent, S>): Promise<S>
}

interface Save {
    (infra: StartContinuousRenewInfra): SaveAuthProfileMethod
}
export const saveAuthProfile: Save = (infra) => async (profile, post) => {
    const { profileRepository } = infra

    const authnResult = await profileRepository.set(profile)
    if (!authnResult.success) {
        return post({ type: "failed-to-save", err: authnResult.err })
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
        const { clock, config, profileRepository, renewRemote } = infra

        const result = await profileRepository.get()
        if (!result.success) {
            return { type: "repository-error", continue: false, err: result.err }
        }
        if (!result.found) {
            return clearProfile()
        }

        // 前回の更新時刻が新しければ今回は通信しない
        const time = { now: clock.now(), ...config.authnExpire }
        if (!hasExpired(result.value, time)) {
            return { type: "authn-not-expired", continue: true }
        }

        const response = await renewRemote()
        if (!response.success) {
            if (response.err.type === "unauthorized") {
                return clearProfile()
            } else {
                return { type: "failed-to-renew", continue: false, err: response.err }
            }
        }

        const saveProfileResult = await profileRepository.set(response.value)
        if (!saveProfileResult.success) {
            return { type: "repository-error", continue: false, err: saveProfileResult.err }
        }

        return { type: "succeed-to-renew", continue: true }

        async function clearProfile(): Promise<StartContinuousRenewEvent> {
            const removeProfileResult = await profileRepository.remove()
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
