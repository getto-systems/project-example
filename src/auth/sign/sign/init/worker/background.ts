import { newRequestResetTokenWorkerHandler } from "../../../../user/password/reset/request_token/init/worker/background"

import { WorkerBackgroundHandler } from "../../../../../z_vendor/getto-application/action/worker/background"

import { SignForegroundMessage, SignBackgroundMessage } from "./message"
import {
    RequestResetTokenProxyMessage,
    RequestResetTokenProxyResponse,
} from "../../../../user/password/reset/request_token/init/worker/message"

import { WorkerOutsideFeature } from "../../../../../z_vendor/getto-application/action/worker/feature"

type OutsideFeature = WorkerOutsideFeature
export function newSignViewWorkerBackground(feature: OutsideFeature): void {
    const { worker } = feature

    const handler: Handler = {
        password: {
            reset: {
                requestToken: newRequestResetTokenWorkerHandler((response) =>
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
    return async (message): Promise<true> => {
        try {
            await handler.password.reset.requestToken(message.message)
            return true
        } catch (err) {
            errorHandler(`${err}`)
            return true
        }
    }
}

interface Post<M> {
    (message: M): void
}
