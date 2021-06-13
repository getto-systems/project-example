import { passThroughRemoteValue } from "../../../z_details/_ui/remote/helper"

import { NotifyUnexpectedErrorInfra } from "./infra"

export interface NotifyUnexpectedErrorMethod {
    (err: unknown): void
}

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
