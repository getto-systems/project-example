import { newWorkerBackgroundOutsideFeature } from "../../../../../x_outside_feature/worker"

import { newChangePasswordWorkerHandler } from "../../../../../auth/user/password/change/detail/worker/background"
import { newRequestResetTokenWorkerHandler } from "../../../../../auth/user/password/reset/request_token/detail/worker/background"

import { WorkerBackgroundHandler } from "../../../../../common/util/worker/background"

import { ProfileForegroundMessage, ProfileBackgroundMessage } from "./message"
import {
    ChangePasswordProxyMessage,
    ChangePasswordProxyResponse,
} from "../../../../../auth/user/password/change/detail/worker/message"
import {
    RequestResetTokenProxyMessage,
    RequestResetTokenProxyResponse,
} from "../../../../../auth/user/password/reset/request_token/detail/worker/message"

newBackground()

function newBackground(): void {
    const feature = newWorkerBackgroundOutsideFeature()
    const { worker } = feature

    const handler: Handler = {
        password: {
            change: newChangePasswordWorkerHandler((response) =>
                postBackgroundMessage({ type: "password-change", response }),
            ),
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

    function postBackgroundMessage(message: ProfileBackgroundMessage) {
        worker.postMessage(message)
    }
}

type Handler = Readonly<{
    password: Readonly<{
        change: WorkerBackgroundHandler<ChangePasswordProxyMessage, ChangePasswordProxyResponse>
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
): Post<ProfileForegroundMessage> {
    return async (message): Promise<true> => {
        try {
            switch (message.type) {
                case "password-change":
                    await handler.password.change(message.message)
                    return true

                case "password-reset-requestToken":
                    await handler.password.reset.requestToken(message.message)
                    return true
            }
        } catch (err) {
            errorHandler(`${err}`)
            return true
        }
    }
}

interface Post<M> {
    (message: M): void
}
