import { delayedChecker } from "../../../z_lib/ui/timer/helper"

import { ApplicationAbstractStateAction } from "../../../../ui/vendor/getto-application/action/init"
import { ApplicationStateAction } from "../../../../ui/vendor/getto-application/action/action"

import { startContinuousRenew } from "../start_continuous_renew/method"
import { getScriptPath } from "../../sign/get_script_path/method"
import { hasExpired } from "../kernel/helper"

import { GetScriptPathConfig, GetScriptPathShell } from "../../sign/get_script_path/infra"
import { AuthTicketRepository, RenewAuthTicketRemote } from "../kernel/infra"
import { Clock } from "../../../z_lib/ui/clock/infra"
import { DelayTime, ExpireTime } from "../../../z_lib/ui/config/infra"
import {
    StartContinuousRenewConfig,
    StartContinuousRenewInfra,
} from "../start_continuous_renew/infra"

import { StartContinuousRenewEvent } from "../start_continuous_renew/event"

import { AuthTicket } from "../kernel/data"
import { ConvertScriptPathResult, LoadScriptError } from "../../sign/get_script_path/data"
import { RepositoryError } from "../../../z_lib/ui/repository/data"
import { RemoteCommonError } from "../../../z_lib/ui/remote/data"

export interface CheckAuthTicketAction extends ApplicationStateAction<CheckAuthTicketState> {
    succeedToInstantLoad(): Promise<CheckAuthTicketState>
    failedToInstantLoad(): Promise<CheckAuthTicketState>
    loadError(err: LoadScriptError): Promise<CheckAuthTicketState>
}

export type CheckAuthTicketInfra = Readonly<{
    ticketRepository: AuthTicketRepository
    renewRemote: RenewAuthTicketRemote
    clock: Clock
}> &
    StartContinuousRenewInfra

export type CheckAuthTicketConfig = Readonly<{
    instantLoadExpire: ExpireTime
    takeLongtimeThreshold: DelayTime
}> &
    StartContinuousRenewConfig &
    GetScriptPathConfig

export type CheckAuthTicketShell = GetScriptPathShell

export type CheckAuthTicketState =
    | Readonly<{ type: "initial-check" }>
    | RenewEvent
    | StartContinuousRenewEvent
    | Readonly<{ type: "try-to-instant-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "try-to-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "load-error"; err: LoadScriptError }>

export const initialCheckAuthTicketState: CheckAuthTicketState = {
    type: "initial-check",
}

export function initCheckAuthTicketAction(
    config: CheckAuthTicketConfig,
    infra: CheckAuthTicketInfra,
    shell: CheckAuthTicketShell,
): CheckAuthTicketAction {
    return new Action(config, infra, shell)
}

class Action
    extends ApplicationAbstractStateAction<CheckAuthTicketState>
    implements CheckAuthTicketAction
{
    readonly initialState = initialCheckAuthTicketState

    config: CheckAuthTicketConfig
    infra: CheckAuthTicketInfra
    shell: CheckAuthTicketShell

    constructor(
        config: CheckAuthTicketConfig,
        infra: CheckAuthTicketInfra,
        shell: CheckAuthTicketShell,
    ) {
        super(async () => {
            const checkResult = await check(this.config, this.infra, this.post)
            if (!checkResult.success) {
                return checkResult.state
            }
            if (!checkResult.expired) {
                return this.post({
                    type: "try-to-instant-load",
                    scriptPath: this.secureScriptPath(),
                })
            }
            return this.startContinuousRenew(checkResult.ticket)
        })
        this.config = config
        this.infra = infra
        this.shell = shell
    }

    succeedToInstantLoad(): Promise<CheckAuthTicketState> {
        return startContinuousRenew(this.config, this.infra, { hold: false }, this.post)
    }
    async failedToInstantLoad(): Promise<CheckAuthTicketState> {
        const result = await renew(this.config, this.infra, this.post)
        if (!result.success) {
            return result.state
        }
        return this.startContinuousRenew(result.ticket)
    }
    async loadError(err: LoadScriptError): Promise<CheckAuthTicketState> {
        return this.post({ type: "load-error", err })
    }

    secureScriptPath() {
        return getScriptPath(this.config, this.shell)
    }

    async startContinuousRenew(ticket: AuthTicket): Promise<CheckAuthTicketState> {
        return await startContinuousRenew(
            this.config,
            this.infra,
            { hold: true, ticket },
            (event) => {
                switch (event.type) {
                    case "succeed-to-start-continuous-renew":
                        return this.post({
                            type: "try-to-load",
                            scriptPath: this.secureScriptPath(),
                        })
                    default:
                        return this.post(event)
                }
            },
        )
    }
}

type RenewEvent =
    | Readonly<{ type: "required-to-login" }>
    | Readonly<{ type: "try-to-renew" }>
    | Readonly<{ type: "take-longtime-to-renew" }>
    | Readonly<{ type: "failed-to-renew"; err: RemoteCommonError }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

type CheckResult<S> =
    | Readonly<{ success: true; expired: true; ticket: AuthTicket }>
    | Readonly<{ success: true; expired: false }>
    | Readonly<{ success: false; state: S }>

async function check<S>(
    config: CheckAuthTicketConfig,
    infra: CheckAuthTicketInfra,
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

    const renewResult = await renew(config, infra, post)
    if (!renewResult.success) {
        return renewResult
    }

    return { success: true, expired: true, ticket: renewResult.ticket }
}

type RenewResult<S> =
    | Readonly<{ success: true; ticket: AuthTicket }>
    | Readonly<{ success: false; state: S }>

async function renew<S>(
    config: CheckAuthTicketConfig,
    infra: CheckAuthTicketInfra,
    post: Post<RenewEvent, S>,
): Promise<RenewResult<S>> {
    const { ticketRepository, renewRemote } = infra

    post({ type: "try-to-renew" })

    // ネットワークの状態が悪い可能性があるので、一定時間後に take longtime イベントを発行
    const response = await delayedChecker(renewRemote(), config.takeLongtimeThreshold, () =>
        post({ type: "take-longtime-to-renew" }),
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
