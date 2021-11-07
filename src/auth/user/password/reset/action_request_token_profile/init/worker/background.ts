import { newRequestResetTokenProfileMaterial } from "../common"

import { WorkerHandler } from "../../../../../../../../ui/vendor/getto-application/action/worker/background"

import {
    RequestResetTokenProfileProxyMessage,
    RequestResetTokenProfileProxyResponse,
} from "./message"

import { RemoteOutsideFeature } from "../../../../../../../z_lib/ui/remote/feature"

type OutsideFeature = RemoteOutsideFeature
export function newRequestResetTokenProfileHandler(
    feature: OutsideFeature,
    post: Post<RequestResetTokenProfileProxyResponse>,
): WorkerHandler<RequestResetTokenProfileProxyMessage> {
    const material = newRequestResetTokenProfileMaterial(feature)
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
