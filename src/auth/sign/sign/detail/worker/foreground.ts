import { RepositoryOutsideFeature } from "../../../../../common/util/repository/feature"
import { WorkerOutsideFeature } from "../../../../../common/util/worker/feature"
import { LocationOutsideFeature } from "../../../../../common/util/location/feature"

import { newSignActionShell } from "../shell"
import { newCheckAuthTicketAction } from "../../../../ticket/authenticate/detail/view"
import { newAuthenticatePasswordAction } from "../../../../user/password/authenticate/detail/resource"
import { newResetPasswordAction } from "../../../../user/password/reset/reset/detail/resource"
import {
    newRequestResetTokenProxy,
    RequestResetTokenProxy,
} from "../../../../user/password/reset/request_token/detail/worker/foreground"
import { newRequestResetTokenAction } from "../../../../user/password/reset/request_token/detail/worker/foreground"

import { initSignAction, SignAction } from "../../action"

import { SignForegroundMessage, SignBackgroundMessage } from "./message"

type OutsideFeature = RepositoryOutsideFeature & WorkerOutsideFeature & LocationOutsideFeature
export function newSignViewWorkerForeground(feature: OutsideFeature): SignAction {
    const { worker } = feature
    const proxy = initProxy(postForegroundMessage)

    const sign = initSignAction(newSignActionShell(feature), {
        check: () => newCheckAuthTicketAction(feature),

        password_authenticate: () => newAuthenticatePasswordAction(feature),
        password_reset_requestToken: () =>
            newRequestResetTokenAction(proxy.password.reset.requestToken.infra),
        password_reset: () => newResetPasswordAction(feature),
    })

    const messageHandler = initBackgroundMessageHandler(proxy, (err: string) => {
        throw new Error(err)
    })

    worker.addEventListener("message", (event) => {
        messageHandler(event.data)
    })

    return sign

    function postForegroundMessage(message: SignForegroundMessage) {
        worker.postMessage(message)
    }
}

type Proxy = Readonly<{
    password: Readonly<{
        reset: Readonly<{
            requestToken: RequestResetTokenProxy
        }>
    }>
}>
function initProxy(post: Post<SignForegroundMessage>): Proxy {
    return {
        password: {
            reset: {
                requestToken: newRequestResetTokenProxy((message) =>
                    post({ type: "password-reset-requestToken", message }),
                ),
            },
        },
    }
}
function initBackgroundMessageHandler(
    proxy: Proxy,
    errorHandler: Post<string>,
): Post<SignBackgroundMessage> {
    return (message): true => {
        try {
            switch (message.type) {
                case "password-reset-requestToken":
                    proxy.password.reset.requestToken.resolve(message.response)
                    return true

                case "error":
                    errorHandler(message.err)
                    return true
            }
        } catch (err) {
            errorHandler(`${err}`)
            return true
        }
    }
}

interface Post<T> {
    (state: T): void
}
