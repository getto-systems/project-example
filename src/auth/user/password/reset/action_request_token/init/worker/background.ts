import { newRequestResetTokenMaterial } from "../common"

import { WorkerHandler } from "../../../../../../../../ui/vendor/getto-application/action/worker/background"

import {
    RequestResetTokenProxyMessage,
    RequestResetTokenProxyResponse,
} from "./message"

import { RemoteOutsideFeature } from "../../../../../../../z_lib/ui/remote/feature"

type OutsideFeature = RemoteOutsideFeature
export function newRequestResetTokenHandler(
    feature: OutsideFeature,
    post: Post<RequestResetTokenProxyResponse>,
): WorkerHandler<RequestResetTokenProxyMessage> {
    const material = newRequestResetTokenMaterial(feature)
    return async (message) => {
        switch (message.method) {
            case "requestToken":
                await material.requestToken(message.params.fields, (event) => {
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
