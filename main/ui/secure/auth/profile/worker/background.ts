import { newWorkerBackgroundOutsideFeature } from "../../../../../../src/x_outside_feature/worker"

import { newRequestResetTokenProfileHandler } from "../../../../../../src/auth/user/password/reset/action_request_token_profile/init/worker/background"

import { WorkerHandler } from "../../../../../../ui/vendor/getto-application/action/worker/background"

import { ProfileForegroundMessage, ProfileBackgroundMessage } from "./message"
import { RequestResetTokenProfileProxyMessage } from "../../../../../../src/auth/user/password/reset/action_request_token_profile/init/worker/message"

newBackground()

function newBackground(): void {
    const feature = newWorkerBackgroundOutsideFeature()
    const { worker } = feature

    const handler: Handler = {
        password: {
            reset: {
                requestToken: newRequestResetTokenProfileHandler(feature, (response) =>
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

    function postBackgroundMessage(message: ProfileBackgroundMessage) {
        worker.postMessage(message)
    }
}

type Handler = Readonly<{
    password: Readonly<{
        reset: Readonly<{
            requestToken: WorkerHandler<RequestResetTokenProfileProxyMessage>
        }>
    }>
}>

function initForegroundMessageHandler(
    handler: Handler,
    errorHandler: Post<string>,
): Post<ProfileForegroundMessage> {
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
