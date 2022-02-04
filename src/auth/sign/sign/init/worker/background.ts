import { newRequestResetTokenWorkerHandler } from "../../../../user/password/reset/request_token/init/worker/background"

import { WorkerBackgroundHandler } from "../../../../../z_vendor/getto-application/action/worker/background"

import { SignForegroundMessage, SignBackgroundMessage } from "./message"
import {
    RequestResetTokenProxyMessage,
    RequestResetTokenProxyResponse,
} from "../../../../user/password/reset/request_token/init/worker/message"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"
import { WorkerOutsideFeature } from "../../../../../z_vendor/getto-application/action/worker/feature"

type OutsideFeature = RemoteOutsideFeature & WorkerOutsideFeature
export function newSignViewWorkerBackground(feature: OutsideFeature): void {
    const { worker } = feature

    const handler: Handler = {
        password: {
            reset: {
                requestToken: newRequestResetTokenWorkerHandler(feature, (response) =>
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

    function postBackgroundMessage(message: SignBackgroundMessage) {
        worker.postMessage(message)
    }
}

type Handler = Readonly<{
    password: Readonly<{
        reset: Readonly<{
            requestToken: WorkerBackgroundHandler<
                RequestResetTokenProxyMessage,
                RequestResetTokenProxyResponse
            >
        }>
    }>
}>

function initForegroundMessageHandler(
    handler: Handler,
    errorHandler: Post<string>,
): Post<SignForegroundMessage> {
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
