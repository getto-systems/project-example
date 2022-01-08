import { NotifyUnexpectedErrorRemote } from "./infra"

export type NotifyUnexpectedErrorInfra = Readonly<{
    notify: NotifyUnexpectedErrorRemote
}>

export interface NotifyUnexpectedErrorAction {
    notify(err: unknown): void
}

export function initNotifyUnexpectedErrorAction(
    infra: NotifyUnexpectedErrorInfra,
): NotifyUnexpectedErrorAction {
    return {
        async notify(err) {
            const result = await infra.notify(err)
            if (!result.success) {
                // エラーの通知に失敗したらもうどうしようもないので console.log しておく
                console.log(result.err)
            }
        },
    }
}
