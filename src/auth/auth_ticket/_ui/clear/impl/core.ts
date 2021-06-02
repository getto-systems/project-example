import { passThroughRemoteValue } from "../../../../../../ui/vendor/getto-application/infra/remote/helper"
import { authzRepositoryConverter } from "../../kernel/converter"
import { authnRepositoryConverter } from "../../kernel/converter"

import { ClearAuthTicketInfra } from "../infra"

import { ClearAuthTicketMethod } from "../method"

interface Clear {
    (infra: ClearAuthTicketInfra): ClearAuthTicketMethod
}
export const clearAuthTicket: Clear = (infra) => async (post) => {
    const authn = infra.authn(authnRepositoryConverter)
    const authz = infra.authz(authzRepositoryConverter)
    const clear = infra.clear(passThroughRemoteValue)

    const authnResult = await authn.get()
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

    const clearResponse = await clear({ type: "always" })
    if (!clearResponse.success) {
        return post({ type: "failed-to-clear", err: clearResponse.err })
    }

    const authnRemoveResult = await authn.remove()
    if (!authnRemoveResult.success) {
        return post({ type: "failed-to-logout", err: authnRemoveResult.err })
    }

    const authzRemoveResult = await authz.remove()
    if (!authzRemoveResult.success) {
        return post({ type: "failed-to-logout", err: authzRemoveResult.err })
    }

    return post({ type: "succeed-to-logout" })
}
