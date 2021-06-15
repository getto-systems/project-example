import { ClearAuthTicketInfra } from "./infra"

import { ClearAuthTicketEvent } from "./event"

import { authzRepositoryConverter } from "../kernel/converter"

export interface ClearAuthTicketMethod {
    <S>(post: Post<ClearAuthTicketEvent, S>): Promise<S>
}

interface Clear {
    (infra: ClearAuthTicketInfra): ClearAuthTicketMethod
}
export const clearAuthTicket: Clear = (infra) => async (post) => {
    const authz = infra.authz(authzRepositoryConverter)

    const authnResult = await infra.authn.get()
    if (!authnResult.success) {
        return post({ type: "failed-to-logout", err: authnResult.err })
    }
    if (!authnResult.found) {
        // authn が保存されていなければ authz のクリアだけ行う
        const authzRemoveResult = await authz.remove()
        if (!authzRemoveResult.success) {
            return post({ type: "failed-to-logout", err: authzRemoveResult.err })
        }

        return post({ type: "succeed-to-logout" })
    }

    const clearResponse = await infra.clear()
    if (!clearResponse.success) {
        return post({ type: "failed-to-clear", err: clearResponse.err })
    }

    const authnRemoveResult = await infra.authn.remove()
    if (!authnRemoveResult.success) {
        return post({ type: "failed-to-logout", err: authnRemoveResult.err })
    }

    const authzRemoveResult = await authz.remove()
    if (!authzRemoveResult.success) {
        return post({ type: "failed-to-logout", err: authzRemoveResult.err })
    }

    return post({ type: "succeed-to-logout" })
}

interface Post<E, S> {
    (event: E): S
}
