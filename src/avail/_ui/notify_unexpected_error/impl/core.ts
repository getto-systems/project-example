import { passThroughRemoteValue } from "../../../../../ui/vendor/getto-application/infra/remote/helper"

import { NotifyUnexpectedErrorInfra } from "../infra"

import { NotifyUnexpectedErrorMethod } from "../method"

interface Notify {
    (infra: NotifyUnexpectedErrorInfra): NotifyUnexpectedErrorMethod
}
export const notifyUnexpectedError: Notify = (infra) => async (err) => {
    const notify = infra.notify(passThroughRemoteValue)

    const result = await notify(err)
    if (!result.success) {
        // エラーの通知に失敗したらもうどうしようもないので console.log しておく
        console.log(result.err)
    }
}
