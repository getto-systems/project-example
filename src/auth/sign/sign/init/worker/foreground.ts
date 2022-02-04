import { toApplicationView } from "../../../../../z_vendor/getto-application/action/helper"

import { RepositoryOutsideFeature } from "../../../../../z_lib/ui/repository/feature"
import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"
import { WorkerOutsideFeature } from "../../../../../z_vendor/getto-application/action/worker/feature"
import { LocationOutsideFeature } from "../../../../../z_lib/ui/location/feature"

import { newSignActionShell } from "../shell"
import { newCheckAuthTicketView } from "../../../../ticket/check/init/view"
import { newAuthenticatePasswordView } from "../../../../user/password/authenticate/init/view"
import { newResetPasswordView } from "../../../../user/password/reset/reset/init/view"
import {
    newRequestResetTokenProxy,
    RequestResetTokenProxy,
} from "../../../../user/password/reset/request_token/init/worker/foreground"
import { initRequestResetTokenView } from "../../../../user/password/reset/request_token/init/worker/foreground"

import { initSignAction, SignAction } from "../../action"
import { initSignLinkResource } from "../../../nav/resource"

import { SignForegroundMessage, SignBackgroundMessage } from "./message"
import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"

type OutsideFeature = RemoteOutsideFeature &
    RepositoryOutsideFeature &
    WorkerOutsideFeature &
    LocationOutsideFeature
export function newSignViewWorkerForeground(feature: OutsideFeature): ApplicationView<SignAction> {
    const { worker } = feature
    const proxy = initProxy(postForegroundMessage)

    const sign = initSignAction(newSignActionShell(feature), {
        link: () => initSignLinkResource(),

        check: () => newCheckAuthTicketView(feature),

        password_authenticate: () => newAuthenticatePasswordView(feature),
        password_reset_requestToken: () =>
            initRequestResetTokenView(proxy.password.reset.requestToken.infra),
        password_reset: () => newResetPasswordView(feature),
    })

    const messageHandler = initBackgroundMessageHandler(proxy, (err: string) => {
        throw new Error(err)
    })

    worker.addEventListener("message", (event) => {
        messageHandler(event.data)
    })

    const view = toApplicationView(sign)
    return {
        resource: view.resource,
        terminate: () => {
            worker.terminate()
            view.terminate()
        },
    }

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
