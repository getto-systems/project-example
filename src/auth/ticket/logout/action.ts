import {
    ApplicationState,
    initApplicationState,
} from "../../../z_vendor/getto-application/action/action"

import { AuthTicketRepository } from "../kernel/infra"
import { LogoutRemote } from "./infra"

import { RepositoryError } from "../../../z_lib/ui/repository/data"
import { RemoteCommonError } from "../../../z_lib/ui/remote/data"

export interface LogoutAction {
    readonly state: ApplicationState<LogoutState>
    submit(): Promise<LogoutState>
}

export type LogoutState = Readonly<{ type: "initial" }> | LogoutEvent

const initialState: LogoutState = { type: "initial" }

export type LogoutInfra = Readonly<{
    ticketRepository: AuthTicketRepository
    logoutRemote: LogoutRemote
}>

export function initLogoutAction(infra: LogoutInfra): LogoutAction {
    const { state, post } = initApplicationState({ initialState })
    return {
        state,
        submit(): Promise<LogoutState> {
            return logout(infra, post)
        },
    }
}

type LogoutEvent =
    | Readonly<{ type: "repository-error"; err: RepositoryError }>
    | Readonly<{ type: "failed"; err: RemoteCommonError }>
    | Readonly<{ type: "success" }>

async function logout<S>(infra: LogoutInfra, post: Post<LogoutEvent, S>): Promise<S> {
    const { ticketRepository, logoutRemote } = infra

    const findResult = await ticketRepository.get()
    if (!findResult.success) {
        return post({ type: "repository-error", err: findResult.err })
    }
    if (!findResult.found) {
        // 認証情報のクリアをするのが目的なので、ticket が設定されていなければ success とする
        return post({ type: "success" })
    }

    const response = await logoutRemote()
    if (!response.success) {
        return post({ type: "failed", err: response.err })
    }

    const removeResult = await ticketRepository.remove()
    if (!removeResult.success) {
        return post({ type: "repository-error", err: removeResult.err })
    }

    return post({ type: "success" })
}

interface Post<E, S> {
    (event: E): S
}
