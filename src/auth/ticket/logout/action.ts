import {
    ApplicationState,
    initApplicationState,
    StatefulApplicationAction,
} from "../../../z_vendor/getto-application/action/action"

import { AuthTicketRepository } from "../kernel/infra"
import { LogoutRemote } from "./infra"

import { RepositoryError } from "../../../z_lib/ui/repository/data"
import { RemoteCommonError } from "../../../z_lib/ui/remote/data"

export interface LogoutAction extends StatefulApplicationAction<LogoutState> {
    submit(): Promise<LogoutState>
}

export type LogoutState =
    | Readonly<{ type: "initial" }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>
    | Readonly<{ type: "failed"; err: RemoteCommonError }>
    | Readonly<{ type: "success" }>

const initialState: LogoutState = { type: "initial" }

export function initLogoutAction(infra: LogoutInfra): LogoutAction {
    return new Action(infra)
}

export type LogoutInfra = Readonly<{
    ticketRepository: AuthTicketRepository
    logoutRemote: LogoutRemote
}>

class Action implements LogoutAction {
    readonly infra: LogoutInfra
    readonly state: ApplicationState<LogoutState>
    readonly post: (state: LogoutState) => LogoutState

    constructor(infra: LogoutInfra) {
        const { state, post } = initApplicationState({ initialState })
        this.infra = infra
        this.state = state
        this.post = post
    }

    async submit(): Promise<LogoutState> {
        const { ticketRepository, logoutRemote } = this.infra

        const findProfileResult = await ticketRepository.get()
        if (!findProfileResult.success) {
            return this.post({ type: "repository-error", err: findProfileResult.err })
        }
        if (!findProfileResult.found) {
            // 認証情報のクリアをするのが目的なので、profile が設定されていなければ success とする
            return this.post({ type: "success" })
        }

        const response = await logoutRemote()
        if (!response.success) {
            return this.post({ type: "failed", err: response.err })
        }

        const removeProfileResult = await ticketRepository.remove()
        if (!removeProfileResult.success) {
            return this.post({ type: "repository-error", err: removeProfileResult.err })
        }

        return this.post({ type: "success" })
    }
}
