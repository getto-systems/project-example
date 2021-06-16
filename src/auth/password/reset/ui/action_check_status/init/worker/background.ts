import { WorkerHandler } from "../../../../../../../../ui/vendor/getto-application/action/worker/background"

import { newCheckSendingStatusMaterialPod } from "../common"

import {
    CheckPasswordResetSendingStatusProxyMessage,
    CheckPasswordResetSendingStatusProxyResponse,
} from "./message"

import { RemoteOutsideFeature } from "../../../../../../../z_details/_ui/remote/feature"

type OutsideFeature = RemoteOutsideFeature
export function newCheckPasswordResetSendingStatusWorkerHandler(
    feature: OutsideFeature,
    post: Post<CheckPasswordResetSendingStatusProxyResponse>,
): WorkerHandler<CheckPasswordResetSendingStatusProxyMessage> {
    const pod = newCheckSendingStatusMaterialPod(feature)
    return async (message) => {
        switch (message.method) {
            case "checkStatus":
                await pod.initCheckStatus(() => message.params)((event) => {
                    post({ ...message, done: false, event })
                })
                post({ ...message, done: true })
                return
        }
    }
}

interface Post<R> {
    (response: R): void
}
