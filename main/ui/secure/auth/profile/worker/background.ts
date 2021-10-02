import { newWorkerBackgroundOutsideFeature } from "../../../../../../src/x_outside_feature/worker"

import { newChangePasswordHandler } from "../../../../../../src/auth/user/password/action_change/init/worker/background"
import { newRequestResetTokenProfileHandler } from "../../../../../../src/auth/user/password/reset/action_request_token_profile/init/worker/background"

import { WorkerHandler } from "../../../../../../ui/vendor/getto-application/action/worker/background"

import { ProfileForegroundMessage, ProfileBackgroundMessage } from "./message"
import { ChangePasswordProxyMessage } from "../../../../../../src/auth/user/password/action_change/init/worker/message"
import { RequestResetTokenProfileProxyMessage } from "../../../../../../src/auth/user/password/reset/action_request_token_profile/init/worker/message"

newBackground()

function newBackground(): void {
    const feature = newWorkerBackgroundOutsideFeature()
    const { worker } = feature

    const handler: Handler = {
        password: {
            change: newChangePasswordHandler(feature, (response) =>
                postBackgroundMessage({ type: "password-change", response }),
            ),
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
        change: WorkerHandler<ChangePasswordProxyMessage>
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
            switch (message.type) {
                case "password-change":
                    handler.password.change(message.message)
                    return

                case "password-reset-requestToken":
                    handler.password.reset.requestToken(message.message)
                    return
            }
        } catch (err) {
            errorHandler(`${err}`)
        }
    }
}

interface Post<M> {
    (message: M): void
}
