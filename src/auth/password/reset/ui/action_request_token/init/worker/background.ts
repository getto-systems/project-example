import { newRequestResetTokenCoreMaterial } from "../common"

import { WorkerHandler } from "../../../../../../../../ui/vendor/getto-application/action/worker/background"

import {
    RequestPasswordResetTokenProxyMessage,
    RequestPasswordResetTokenProxyResponse,
} from "./message"

import { RemoteOutsideFeature } from "../../../../../../../../ui/vendor/getto-application/infra/remote/infra"

type OutsideFeature = RemoteOutsideFeature
export function newRequestResetTokenHandler(
    feature: OutsideFeature,
    post: Post<RequestPasswordResetTokenProxyResponse>,
): WorkerHandler<RequestPasswordResetTokenProxyMessage> {
    const material = newRequestResetTokenCoreMaterial(feature)
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
