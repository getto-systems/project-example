import { h, render } from "preact"

import { ApplicationErrorComponent } from "../../../../../../src/avail/x_preact/application_error"
import { ProfilePageEntry } from "../page"

import { newWorkerForegroundOutsideFeature } from "../../../../../../src/x_outside_feature/worker"

import { newBaseResource } from "../../../../../../src/example/action_base/init/resource"
import { newChangePasswordResource } from "../../../../../../src/auth/user/password/action_change/init/resource"
import {
    newRequestResetTokenProfileProxy,
    RequestResetTokenProfileProxy,
} from "../../../../../../src/auth/user/password/reset/action_request_token_profile/init/worker/foreground"
import { initRequestResetTokenProfileAction } from "../../../../../../src/auth/user/password/reset/action_request_token_profile/init"
import { toProfileView } from "../common"

import { ApplicationView } from "../../../../../../ui/vendor/getto-application/action/action"
import { ProfilePageResource } from "../resource"
import { ProfileBackgroundMessage, ProfileForegroundMessage } from "./message"

renderEntry()

async function renderEntry() {
    try {
        render(h(ProfilePageEntry, await props()), document.body)
    } catch (err) {
        render(h(ApplicationErrorComponent, { err: `${err}` }), document.body)
    }
}

async function props(): Promise<ApplicationView<ProfilePageResource>> {
    const { resource, terminate } = await newResource()
    return toProfileView(resource, terminate)
}
async function newResource() {
    const feature = await newWorkerForegroundOutsideFeature()
    const { worker } = feature
    const proxy = initProxy(postForegroundMessage)

    const messageHandler = initBackgroundMessageHandler(proxy, (err: string) => {
        throw new Error(err)
    })

    worker.addEventListener("message", (event) => {
        messageHandler(event.data)
    })

    return {
        resource: {
            ...newBaseResource(feature),
            ...newChangePasswordResource(feature),
            requestToken: initRequestResetTokenProfileAction(
                proxy.password.reset.requestToken.material(),
            ),
        },
        terminate: () => {
            worker.terminate()
        },
    }

    function postForegroundMessage(message: ProfileForegroundMessage) {
        worker.postMessage(message)
    }
}

type Proxy = Readonly<{
    password: Readonly<{
        reset: Readonly<{
            requestToken: RequestResetTokenProfileProxy
        }>
    }>
}>
function initProxy(post: Post<ProfileForegroundMessage>): Proxy {
    return {
        password: {
            reset: {
                requestToken: newRequestResetTokenProfileProxy((message) =>
                    post({ type: "password-reset-requestToken", message }),
                ),
            },
        },
    }
}
function initBackgroundMessageHandler(
    proxy: Proxy,
    errorHandler: Post<string>,
): Post<ProfileBackgroundMessage> {
    return (message) => {
        try {
            switch (message.type) {
                case "password-reset-requestToken":
                    proxy.password.reset.requestToken.resolve(message.response)
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
