import { checkTakeLongtime } from "../../../common/util/timer/helper"

import { Atom, initAtom } from "../../../z_vendor/getto-atom/atom"

import { startContinuousRenew } from "./method"
import { getScriptPath } from "../../sign/get_script_path/method"
import { hasExpired } from "../kernel/helper"

import {
    StartContinuousRenewConfig,
    StartContinuousRenewInfra,
    StartContinuousRenewEvent,
} from "./method"

import { GetScriptPathConfig, GetScriptPathShell } from "../../sign/get_script_path/infra"
import { AuthTicketRepository } from "../kernel/infra"
import { CheckAuthTicketRemote } from "./infra"
import { Clock } from "../../../common/util/clock/infra"
import { WaitTime, ExpireTime } from "../../../common/util/config/infra"

import { AuthTicket } from "../kernel/data"
import { ConvertScriptPathResult, LoadScriptError } from "../../sign/get_script_path/data"
import { RepositoryError } from "../../../common/util/repository/data"
import { RemoteCommonError } from "../../../common/util/remote/data"

export interface AuthenticateWithTokenAction {
    readonly state: Atom<AuthenticateWithTokenState>
    succeedToInstantLoad(): Promise<AuthenticateWithTokenState>
    failedToInstantLoad(): Promise<AuthenticateWithTokenState>
    loadError(err: LoadScriptError): Promise<AuthenticateWithTokenState>
}

export type AuthenticateWithTokenMaterial = Readonly<{
    infra: AuthenticateWithTokenInfra
    shell: AuthenticateWithTokenShell
    config: AuthenticateWithTokenConfig
}>

export type AuthenticateWithTokenInfra = Readonly<{
    ticketRepository: AuthTicketRepository
    renewRemote: CheckAuthTicketRemote
    clock: Clock
}> &
    StartContinuousRenewInfra

export type AuthenticateWithTokenShell = GetScriptPathShell

export type AuthenticateWithTokenConfig = Readonly<{
    instantLoadExpire: ExpireTime
    takeLongtimeThreshold: WaitTime
}> &
    StartContinuousRenewConfig &
    GetScriptPathConfig

export type AuthenticateWithTokenState =
    | Readonly<{ type: "initial-check" }>
    | RenewEvent
    | StartContinuousRenewEvent
    | Readonly<{ type: "try-to-instant-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "try-to-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "load-error"; err: LoadScriptError }>

const initialState: AuthenticateWithTokenState = { type: "initial-check" }

export function initAuthenticateWithTokenAction(
    material: AuthenticateWithTokenMaterial,
): AuthenticateWithTokenAction {
    const { state, post } = initAtom({
        initialState,
        ignite: async (): Promise<AuthenticateWithTokenState> => {
            const result = await check(material, post)
            if (!result.success) {
                return result.state
            }
            if (!result.expired) {
                return post({ type: "try-to-instant-load", scriptPath: scriptPath() })
            }
            return start(result.ticket)
        },
    })

    return {
        state,
        succeedToInstantLoad(): Promise<AuthenticateWithTokenState> {
            return startContinuousRenew(material, { hasTicket: false }, post)
        },
        async failedToInstantLoad(): Promise<AuthenticateWithTokenState> {
            const result = await renew(material, post)
            if (!result.success) {
                return result.state
            }
            return start(result.ticket)
        },
        async loadError(err: LoadScriptError): Promise<AuthenticateWithTokenState> {
            return post({ type: "load-error", err })
        },
    }

    function scriptPath() {
        return getScriptPath(material)
    }

    async function start(ticket: AuthTicket): Promise<AuthenticateWithTokenState> {
        return await startContinuousRenew(material, { hasTicket: true, ticket }, (event) => {
            switch (event.type) {
                case "succeed-to-start-continuous-renew":
                    return post({ type: "try-to-load", scriptPath: scriptPath() })
                default:
                    return post(event)
            }
        })
    }
}

type CheckMaterial = Readonly<{
    infra: AuthenticateWithTokenInfra
    config: AuthenticateWithTokenConfig
}>

type RenewEvent =
    | Readonly<{ type: "required-to-login" }>
    | Readonly<{ type: "try-to-renew"; hasTakenLongtime: boolean }>
    | Readonly<{ type: "failed-to-renew"; err: RemoteCommonError }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

type CheckResult<S> =
    | Readonly<{ success: true; expired: true; ticket: AuthTicket }>
    | Readonly<{ success: true; expired: false }>
    | Readonly<{ success: false; state: S }>

async function check<S>(
    { infra, config }: CheckMaterial,
    post: Post<RenewEvent, S>,
): Promise<CheckResult<S>> {
    const { clock, ticketRepository } = infra

    const findResult = await ticketRepository.get()
    if (!findResult.success) {
        return { success: false, state: post({ type: "repository-error", err: findResult.err }) }
    }
    if (!findResult.found) {
        return { success: false, state: post({ type: "required-to-login" }) }
    }

    const time = {
        now: clock.now(),
        expire_millisecond: config.instantLoadExpire.expire_millisecond,
    }
    if (!hasExpired(findResult.value, time)) {
        return { success: true, expired: false }
    }

    const renewResult = await renew({ config, infra }, post)
    if (!renewResult.success) {
        return renewResult
    }

    return { success: true, expired: true, ticket: renewResult.ticket }
}

type RenewMaterial = Readonly<{
    config: AuthenticateWithTokenConfig
    infra: AuthenticateWithTokenInfra
}>

type RenewResult<S> =
    | Readonly<{ success: true; ticket: AuthTicket }>
    | Readonly<{ success: false; state: S }>

async function renew<S>(
    { infra, config }: RenewMaterial,
    post: Post<RenewEvent, S>,
): Promise<RenewResult<S>> {
    const { ticketRepository, renewRemote } = infra

    post({ type: "try-to-renew", hasTakenLongtime: false })

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await checkTakeLongtime(renewRemote(), config.takeLongtimeThreshold, () =>
        post({ type: "try-to-renew", hasTakenLongtime: true }),
    )
    if (!response.success) {
        if (response.err.type === "unauthorized") {
            const removeResult = await ticketRepository.remove()
            if (!removeResult.success) {
                return {
                    success: false,
                    state: post({ type: "repository-error", err: removeResult.err }),
                }
            }
            return { success: false, state: post({ type: "required-to-login" }) }
        }
        return { success: false, state: post({ type: "failed-to-renew", err: response.err }) }
    }

    const ticketResult = await ticketRepository.set(response.value)
    if (!ticketResult.success) {
        return { success: false, state: post({ type: "repository-error", err: ticketResult.err }) }
    }

    return { success: true, ticket: response.value }
}

interface Post<E, S> {
    (event: E): S
}
