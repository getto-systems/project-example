import { NotifyUnexpectedErrorInfra } from "./infra"

export interface NotifyUnexpectedErrorMethod {
    (err: unknown): void
}

interface Notify {
    (infra: NotifyUnexpectedErrorInfra): NotifyUnexpectedErrorMethod
}
export const notifyUnexpectedError: Notify = (infra) => async (err) => {
    const result = await infra.notify(err)
    if (!result.success) {
        // エラーの通知に失敗したらもうどうしようもないので console.log しておく
        console.log(result.err)
    }
}
