import { newRequestResetTokenHandler } from "../../../../password/reset/ui/action_request_token/init/worker/background"
import { newCheckPasswordResetSendingStatusWorkerHandler } from "../../../../password/reset/ui/action_check_status/init/worker/background"

import { WorkerHandler } from "../../../../../../ui/vendor/getto-application/action/worker/background"

import { ForegroundMessage, BackgroundMessage } from "./message"
import { RequestPasswordResetTokenProxyMessage } from "../../../../password/reset/ui/action_request_token/init/worker/message"
import { CheckPasswordResetSendingStatusProxyMessage } from "../../../../password/reset/ui/action_check_status/init/worker/message"

import { RemoteOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/remote/feature"
import { WorkerOutsideFeature } from "../../../../../../ui/vendor/getto-application/action/worker/infra"

type OutsideFeature = RemoteOutsideFeature & WorkerOutsideFeature
export function newSignWorkerBackground(feature: OutsideFeature): void {
    const { worker } = feature

    const handler: Handler = {
        password: {
            reset: {
                requestToken: newRequestResetTokenHandler(feature, (response) =>
                    postBackgroundMessage({ type: "password-reset-requestToken", response }),
                ),
                checkStatus: newCheckPasswordResetSendingStatusWorkerHandler(feature, (response) =>
                    postBackgroundMessage({ type: "password-reset-checkStatus", response }),
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
            checkStatus: WorkerHandler<CheckPasswordResetSendingStatusProxyMessage>
        }>
    }>
}>

function initForegroundMessageHandler(
    handler: Handler,
    errorHandler: Post<string>,
): Post<ForegroundMessage> {
    return (message) => {
        try {
            switch (message.type) {
                case "password-reset-requestToken":
                    handler.password.reset.requestToken(message.message)
                    break

                case "password-reset-checkStatus":
                    handler.password.reset.checkStatus(message.message)
                    break

                default:
                    assertNever(message)
            }
        } catch (err) {
            errorHandler(`${err}`)
        }
    }
}

interface Post<M> {
    (message: M): void
}

function assertNever(_: never): never {
    throw new Error("NEVER")
}
