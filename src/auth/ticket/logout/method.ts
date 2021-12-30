import { LogoutInfra } from "./infra"

import { LogoutEvent } from "./event"

export interface LogoutMethod {
    <S>(post: Post<LogoutEvent, S>): Promise<S>
}

interface Logout {
    (infra: LogoutInfra): LogoutMethod
}
export const logout: Logout = (infra) => async (post) => {
    const { profileRepository, logoutRemote } = infra

    const findProfileResult = await profileRepository.get()
    if (!findProfileResult.success) {
        return post({ type: "repository-error", err: findProfileResult.err })
    }
    if (!findProfileResult.found) {
        // 認証情報のクリアをするのが目的なので、profile が設定されていなければ success とする
        return post({ type: "succeed-to-logout" })
    }

    const response = await logoutRemote()
    if (!response.success) {
        return post({ type: "failed-to-logout", err: response.err })
    }

    const removeProfileResult = await profileRepository.remove()
    if (!removeProfileResult.success) {
        return post({ type: "repository-error", err: removeProfileResult.err })
    }

    return post({ type: "succeed-to-logout" })
}

interface Post<E, S> {
    (event: E): S
}
