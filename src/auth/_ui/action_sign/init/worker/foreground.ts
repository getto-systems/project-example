import { newCheckAuthTicketView } from "../../../../auth_ticket/_ui/action_check/init"
import { newSignViewLocationDetecter } from "../../../common/switch_view/init"
import { newAuthenticatePasswordView } from "../../../../password/_ui/action_authenticate/init"
import { newResetPasswordView } from "../../../../password/reset/ui/action_reset/init"
import {
    newRequestPasswordResetTokenProxy,
    RequestPasswordResetTokenProxy,
} from "../../../../password/reset/ui/action_request_token/init/worker/foreground"
import {
    CheckPasswordResetSendingStatusProxy,
    newCheckPasswordResetSendingStatusProxy,
} from "../../../../password/reset/ui/action_check_status/init/worker/foreground"

import { initSignView } from "../../impl"
import { initSignAction } from "../../core/impl"
import { initSignLinkResource } from "../../../common/nav/action_nav/impl"

import { ForegroundMessage, BackgroundMessage } from "./message"

import { RepositoryOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/repository/feature"
import { RemoteOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/remote/feature"
import { WorkerOutsideFeature } from "../../../../../../ui/vendor/getto-application/action/worker/infra"
import { LocationOutsideFeature } from "../../../../../../ui/vendor/getto-application/location/infra"

import { SignView } from "../../resource"

type OutsideFeature = RemoteOutsideFeature &
    RepositoryOutsideFeature &
    WorkerOutsideFeature &
    LocationOutsideFeature
export function newSignWorkerForeground(feature: OutsideFeature): SignView {
    const { worker } = feature
    const proxy = initProxy(postForegroundMessage)

    const sign = initSignAction(newSignViewLocationDetecter(feature), {
        link: () => initSignLinkResource(),

        check: () => newCheckAuthTicketView(feature),

        password_authenticate: () => newAuthenticatePasswordView(feature),
        password_reset_requestToken: () => proxy.password.reset.requestToken.view(),
        password_reset_checkStatus: () => proxy.password.reset.checkStatus.view(feature),
        password_reset: () => newResetPasswordView(feature),
    })

    const messageHandler = initBackgroundMessageHandler(proxy, (err: string) => {
        sign.error(err)
    })

    worker.addEventListener("message", (event) => {
        messageHandler(event.data)
    })

    const view = initSignView(sign)
    return {
        resource: view.resource,
        terminate: () => {
            worker.terminate()
            view.terminate()
        },
    }

    function postForegroundMessage(message: ForegroundMessage) {
        worker.postMessage(message)
    }
}

type Proxy = Readonly<{
    password: Readonly<{
        reset: Readonly<{
            requestToken: RequestPasswordResetTokenProxy
            checkStatus: CheckPasswordResetSendingStatusProxy
        }>
    }>
}>
function initProxy(post: Post<ForegroundMessage>): Proxy {
    return {
        password: {
            reset: {
                requestToken: newRequestPasswordResetTokenProxy((message) =>
                    post({ type: "password-reset-requestToken", message }),
                ),
                checkStatus: newCheckPasswordResetSendingStatusProxy((message) =>
                    post({ type: "password-reset-checkStatus", message }),
                ),
            },
        },
    }
}
function initBackgroundMessageHandler(
    proxy: Proxy,
    errorHandler: Post<string>,
): Post<BackgroundMessage> {
    return (message) => {
        try {
            switch (message.type) {
                case "password-reset-requestToken":
                    proxy.password.reset.requestToken.resolve(message.response)
                    break

                case "password-reset-checkStatus":
                    proxy.password.reset.checkStatus.resolve(message.response)
                    break

                case "error":
                    errorHandler(message.err)
                    break

                default:
                    assertNever(message)
            }
        } catch (err) {
            errorHandler(`${err}`)
        }
    }
}

function assertNever(_: never): never {
    throw new Error("NEVER")
}

interface Post<T> {
    (state: T): void
}
