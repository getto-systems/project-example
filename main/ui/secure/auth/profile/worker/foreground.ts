import { h, render } from "preact"

import { ApplicationErrorComponent } from "../../../../../../src/avail/x_preact/application_error"
import { ProfilePageEntry } from "../page"

import { newWorkerForegroundOutsideFeature } from "../../../../../../src/x_outside_feature/worker"

import { newBaseResource } from "../../../../../../src/example/action_base/init/resource"
import { newChangePasswordResource } from "../../../../../../src/auth/user/password/action_change/init/resource"
import {
    newRequestResetTokenProxy,
    RequestResetTokenProxy,
} from "../../../../../../src/auth/user/password/reset/request_token/init/worker/foreground"
import { toProfileView } from "../common"

import { ApplicationView } from "../../../../../../ui/vendor/getto-application/action/action"
import { ProfilePageResource } from "../resource"
import { ProfileBackgroundMessage, ProfileForegroundMessage } from "./message"
import { initRequestResetTokenProfileAction } from "../../../../../../src/auth/user/password/reset/request_token/action"
import { newRequestResetTokenConfig } from "../../../../../../src/auth/user/password/reset/request_token/init/config"

renderEntry()

async function renderEntry() {
    try {
        render(h(ProfilePageEntry, await props()), document.body)
    } catch (err) {
        render(h(ApplicationErrorComponent, { err: `${err}` }), document.body)
    }
}

async function props(): Promise<ApplicationView<ProfilePageResource>> {
    return toProfileView(await newResource())
}
async function newResource() {
    const feature = await newWorkerForegroundOutsideFeature()
    const { worker } = feature
    const proxy = initProxy(postForegroundMessage)

    const messageHandler = initBackgroundMessageHandler(proxy)

    worker.addEventListener("message", (event) => {
        messageHandler(event.data)
    })

    return {
        resource: {
            ...newBaseResource(feature),
            ...newChangePasswordResource(feature),
            ...newRequestResetTokenProfileResource(proxy),
        },
        terminate: () => {
            worker.terminate()
        },
    }

    function postForegroundMessage(message: ProfileForegroundMessage) {
        worker.postMessage(message)
    }
}

function newRequestResetTokenProfileResource(proxy: Proxy) {
    return {
        requestToken: initRequestResetTokenProfileAction(
            newRequestResetTokenConfig(),
            proxy.password.reset.requestToken.infra,
        ),
    }
}

type Proxy = Readonly<{
    password: Readonly<{
        reset: Readonly<{
            requestToken: RequestResetTokenProxy
        }>
    }>
}>
function initProxy(post: Post<ProfileForegroundMessage>): Proxy {
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
function initBackgroundMessageHandler(proxy: Proxy): Post<ProfileBackgroundMessage> {
    return (message): true => {
        switch (message.type) {
            case "password-reset-requestToken":
                proxy.password.reset.requestToken.resolve(message.response)
                return true

            case "password-change":
                //proxy.password.change.resolve(message.response)
                return true

            case "error":
                throw new Error(message.err)
        }
    }
}

interface Post<T> {
    (state: T): void
}
