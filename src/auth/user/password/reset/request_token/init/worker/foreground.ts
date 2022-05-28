import { newWorkerProxyMap } from "../../../../../../../z_vendor/getto-application/action/worker/foreground"

import {
    initRequestResetTokenAction,
    RequestResetTokenAction,
    RequestResetTokenInfra,
} from "../../action"

import { RequestResetTokenRemoteResult } from "../../infra"
import { newRequestResetTokenConfig } from "../config"

import { RequestResetTokenProxyMessage, RequestResetTokenProxyResponse } from "./message"

export function newRequestResetTokenAction(infra: RequestResetTokenInfra): RequestResetTokenAction {
    return initRequestResetTokenAction({
        infra,
        config: newRequestResetTokenConfig(),
    })
}

export type RequestResetTokenProxy = Readonly<{
    infra: RequestResetTokenInfra
    resolve: Post<RequestResetTokenProxyResponse>
}>

export function newRequestResetTokenProxy(
    post: Post<RequestResetTokenProxyMessage>,
): RequestResetTokenProxy {
    const map = {
        requestTokenRemote:
            newWorkerProxyMap<Post<RequestResetTokenRemoteResult>>("request-token-remote"),
    }
    return {
        infra: {
            requestTokenRemote: (fields) =>
                new Promise((resolve) => {
                    post({
                        name: "request-token-remote",
                        id: map.requestTokenRemote.register(resolve),
                        params: { fields },
                    })
                }),
        },
        resolve: (response): true => {
            switch (response.name) {
                case "request-token-remote":
                    map.requestTokenRemote.drop(response.id)(response.data)
                    return true
            }
        },
    }
}

interface Post<M> {
    (message: M): void
}
