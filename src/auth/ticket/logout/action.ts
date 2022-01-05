import { ApplicationStateAction } from "../../../../ui/vendor/getto-application/action/action"

import { AuthTicketRepository } from "../kernel/infra"
import { LogoutRemote } from "./infra"

import { RepositoryError } from "../../../z_lib/ui/repository/data"
import { RemoteCommonError } from "../../../z_lib/ui/remote/data"
import { ApplicationAbstractStateAction } from "../../../../ui/vendor/getto-application/action/init"

export interface LogoutAction extends ApplicationStateAction<LogoutState> {
    submit(): Promise<LogoutState>
}

export type LogoutState =
    | Readonly<{ type: "initial-logout" }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>
    | Readonly<{ type: "failed-to-logout"; err: RemoteCommonError }>
    | Readonly<{ type: "succeed-to-logout" }>

export const initialLogoutState: LogoutState = { type: "initial-logout" }

export function initLogoutAction(infra: LogoutInfra): LogoutAction {
    return new Action(infra)
}

export type LogoutInfra = Readonly<{
    ticketRepository: AuthTicketRepository
    logoutRemote: LogoutRemote
}>

class Action extends ApplicationAbstractStateAction<LogoutState> implements LogoutAction {
    readonly initialState = initialLogoutState

    infra: LogoutInfra

    constructor(infra: LogoutInfra) {
        super()
        this.infra = infra
    }

    async submit(): Promise<LogoutState> {
        const { ticketRepository, logoutRemote } = this.infra

        const findProfileResult = await ticketRepository.get()
        if (!findProfileResult.success) {
            return this.post({ type: "repository-error", err: findProfileResult.err })
        }
        if (!findProfileResult.found) {
            // 認証情報のクリアをするのが目的なので、profile が設定されていなければ success とする
            return this.post({ type: "succeed-to-logout" })
        }

        const response = await logoutRemote()
        if (!response.success) {
            return this.post({ type: "failed-to-logout", err: response.err })
        }

        const removeProfileResult = await ticketRepository.remove()
        if (!removeProfileResult.success) {
            return this.post({ type: "repository-error", err: removeProfileResult.err })
        }

        return this.post({ type: "succeed-to-logout" })
    }
}
