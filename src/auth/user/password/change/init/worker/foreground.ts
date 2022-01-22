import { ApplicationView } from "../../../../../../../ui/vendor/getto-application/action/action"
import { toApplicationView } from "../../../../../../../ui/vendor/getto-application/action/helper"
import { newWorkerProxyMap } from "../../../../../../../ui/vendor/getto-application/action/worker/foreground"

import { newChangePasswordConfig } from "../config"

import { ChangePasswordAction, ChangePasswordInfra, initChangePasswordAction } from "../../action"

import { ChangePasswordRemoteResult } from "../../infra"

import { ChangePasswordProxyMessage, ChangePasswordProxyResponse } from "./message"

export function initChangePasswordView(
    infra: ChangePasswordInfra,
): ApplicationView<ChangePasswordAction> {
    return toApplicationView(
        initChangePasswordAction({
            infra,
            config: newChangePasswordConfig(),
        }),
    )
}

export type ChangePasswordProxy = Readonly<{
    infra: ChangePasswordInfra
    resolve: Post<ChangePasswordProxyResponse>
}>

export function newChangePasswordProxy(
    post: Post<ChangePasswordProxyMessage>,
): ChangePasswordProxy {
    const map = {
        changePasswordRemote:
            newWorkerProxyMap<Post<ChangePasswordRemoteResult>>("change-password-remote"),
    }
    return {
        infra: {
            changePasswordRemote: (fields) =>
                new Promise((resolve) => {
                    post({
                        name: "change-password-remote",
                        id: map.changePasswordRemote.register(resolve),
                        params: { fields },
                    })
                }),
        },
        resolve: (response): true => {
            switch (response.name) {
                case "change-password-remote":
                    map.changePasswordRemote.drop(response.id)(response.data)
                    return true
            }
        },
    }
}

interface Post<M> {
    (message: M): void
}
