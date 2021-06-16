import { ClearAuthTicketInfra } from "./infra"

import { ClearAuthTicketEvent } from "./event"

export interface ClearAuthTicketMethod {
    <S>(post: Post<ClearAuthTicketEvent, S>): Promise<S>
}

interface Clear {
    (infra: ClearAuthTicketInfra): ClearAuthTicketMethod
}
export const clearAuthTicket: Clear = (infra) => async (post) => {
    const authnResult = await infra.authn.get()
    if (!authnResult.success) {
        return post({ type: "failed-to-logout", err: authnResult.err })
    }
    if (!authnResult.found) {
        // authn が保存されていなければ authz のクリアだけ行う
        const authzRemoveResult = await infra.authz.remove()
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

    const authzRemoveResult = await infra.authz.remove()
    if (!authzRemoveResult.success) {
        return post({ type: "failed-to-logout", err: authzRemoveResult.err })
    }

    return post({ type: "succeed-to-logout" })
}

interface Post<E, S> {
    (event: E): S
}
