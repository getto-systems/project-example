import { LogoutInfra } from "./infra"

import { LogoutEvent } from "./event"

export interface LogoutMethod {
    <S>(post: Post<LogoutEvent, S>): Promise<S>
}

interface Logout {
    (infra: LogoutInfra): LogoutMethod
}
export const logout: Logout = (infra) => async (post) => {
    const authnResult = await infra.authn.get()
    if (!authnResult.success) {
        return post({ type: "repository-error", err: authnResult.err })
    }
    if (!authnResult.found) {
        // authn が保存されていなければ authz のクリアだけ行う
        const authzRemoveResult = await infra.authz.remove()
        if (!authzRemoveResult.success) {
            return post({ type: "repository-error", err: authzRemoveResult.err })
        }

        // 認証情報のクリアをするのが目的なので、authn は無かったけど success とする
        return post({ type: "succeed-to-logout" })
    }

    const response = await infra.logout()
    if (!response.success) {
        return post({ type: "failed-to-logout", err: response.err })
    }

    const authnRemoveResult = await infra.authn.remove()
    if (!authnRemoveResult.success) {
        return post({ type: "repository-error", err: authnRemoveResult.err })
    }

    const authzRemoveResult = await infra.authz.remove()
    if (!authzRemoveResult.success) {
        return post({ type: "repository-error", err: authzRemoveResult.err })
    }

    return post({ type: "succeed-to-logout" })
}

interface Post<E, S> {
    (event: E): S
}
