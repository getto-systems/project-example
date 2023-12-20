import { h, render } from "preact"

import { ApplicationError } from "../../../../../avail/x_preact/application_error"
import { ProfilePage } from "../page"

import { newWorkerForegroundOutsideFeature } from "../../../../../x_outside_feature/worker"

import { newRequestResetTokenConfig } from "../../../../../auth/user/password/reset/request_token/detail/config"
import { newChangePasswordConfig } from "../../../../../auth/user/password/change/detail/config"

import { newBaseResource } from "../../../base/detail/resource"
import {
    newRequestResetTokenProxy,
    RequestResetTokenProxy,
} from "../../../../../auth/user/password/reset/request_token/detail/worker/foreground"
import {
    ChangePasswordProxy,
    newChangePasswordProxy,
} from "../../../../../auth/user/password/change/detail/worker/foreground"

import { initRequestResetTokenAction } from "../../../../../auth/user/password/reset/request_token/action"
import { initChangePasswordAction } from "../../../../../auth/user/password/change/action"

import { ProfilePageResource } from "../resource"

import { ProfileBackgroundMessage, ProfileForegroundMessage } from "./message"

renderEntry()

async function renderEntry() {
    try {
        render(h(ProfilePage, await props()), document.body)
    } catch (err) {
        render(h(ApplicationError, { err: `${err}` }), document.body)
    }
}

async function props(): Promise<ProfilePageResource> {
    const feature = await newWorkerForegroundOutsideFeature()
    const { worker } = feature
    const proxy = initProxy(postForegroundMessage)

    const messageHandler = initBackgroundMessageHandler(proxy)

    worker.addEventListener("message", (event) => {
        messageHandler(event.data)
    })

    return {
        ...newBaseResource(feature),
        ...newChangePasswordResource(proxy),
        ...newRequestResetTokenResource(proxy),
    }

    function postForegroundMessage(message: ProfileForegroundMessage) {
        worker.postMessage(message)
    }
}

function newChangePasswordResource(proxy: Proxy) {
    return {
        change: initChangePasswordAction({
            infra: proxy.password.change.infra,
            config: newChangePasswordConfig(),
        }),
    }
}
function newRequestResetTokenResource(proxy: Proxy) {
    return {
        requestToken: initRequestResetTokenAction({
            infra: proxy.password.reset.requestToken.infra,
            config: newRequestResetTokenConfig(),
        }),
    }
}

type Proxy = Readonly<{
    password: Readonly<{
        change: ChangePasswordProxy
        reset: Readonly<{
            requestToken: RequestResetTokenProxy
        }>
    }>
}>
function initProxy(post: Post<ProfileForegroundMessage>): Proxy {
    return {
        password: {
            change: newChangePasswordProxy((message) => post({ type: "password-change", message })),
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
                proxy.password.change.resolve(message.response)
                return true

            case "error":
                throw new Error(message.err)
        }
    }
}

interface Post<T> {
    (state: T): void
}
