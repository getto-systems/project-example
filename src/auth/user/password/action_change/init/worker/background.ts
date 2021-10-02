import { newChangePasswordMaterial } from "../common"

import { WorkerHandler } from "../../../../../../../ui/vendor/getto-application/action/worker/background"

import {
    ChangePasswordProxyMessage,
    ChangePasswordProxyResponse,
} from "./message"

import { RemoteOutsideFeature } from "../../../../../../z_lib/ui/remote/feature"

type OutsideFeature = RemoteOutsideFeature
export function newChangePasswordHandler(
    feature: OutsideFeature,
    post: Post<ChangePasswordProxyResponse>,
): WorkerHandler<ChangePasswordProxyMessage> {
    const material = newChangePasswordMaterial(feature)
    return async (message) => {
        switch (message.method) {
            case "change":
                await material.change(message.params.fields, (event) => {
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
