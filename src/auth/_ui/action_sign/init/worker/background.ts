import { newRequestResetTokenHandler } from "../../../../password/reset/_ui/action_request_token/init/worker/background"

import { WorkerHandler } from "../../../../../../ui/vendor/getto-application/action/worker/background"

import { ForegroundMessage, BackgroundMessage } from "./message"
import { RequestPasswordResetTokenProxyMessage } from "../../../../password/reset/_ui/action_request_token/init/worker/message"

import { RemoteOutsideFeature } from "../../../../../z_details/_ui/remote/feature"
import { WorkerOutsideFeature } from "../../../../../../ui/vendor/getto-application/action/worker/feature"

type OutsideFeature = RemoteOutsideFeature & WorkerOutsideFeature
export function newSignWorkerBackground(feature: OutsideFeature): void {
    const { worker } = feature

    const handler: Handler = {
        password: {
            reset: {
                requestToken: newRequestResetTokenHandler(feature, (response) =>
                    postBackgroundMessage({ type: "password-reset-requestToken", response }),
                ),
            },
        },
    }

    const messageHandler = initForegroundMessageHandler(handler, (err: string) => {
        postBackgroundMessage({ type: "error", err })
    })

    worker.addEventListener("message", (event) => {
        messageHandler(event.data)
    })

    function postBackgroundMessage(message: BackgroundMessage) {
        worker.postMessage(message)
    }
}

type Handler = Readonly<{
    password: Readonly<{
        reset: Readonly<{
            requestToken: WorkerHandler<RequestPasswordResetTokenProxyMessage>
        }>
    }>
}>

function initForegroundMessageHandler(
    handler: Handler,
    errorHandler: Post<string>,
): Post<ForegroundMessage> {
    return (message) => {
        try {
            handler.password.reset.requestToken(message.message)
        } catch (err) {
            errorHandler(`${err}`)
        }
    }
}

interface Post<M> {
    (message: M): void
}
