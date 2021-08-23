import { toApplicationView } from "../../../../../../../../ui/vendor/getto-application/action/helper"

import { initRequestResetTokenAction } from "../../init"

import {
    WorkerAbstractProxy,
    WorkerProxy,
} from "../../../../../../../../ui/vendor/getto-application/action/worker/foreground"

import {
    RequestPasswordResetTokenProxyMaterial,
    RequestPasswordResetTokenProxyMessage,
    RequestPasswordResetTokenProxyResponse,
} from "./message"

import { RequestResetTokenView } from "../../resource"

export interface RequestPasswordResetTokenProxy
    extends WorkerProxy<
        RequestPasswordResetTokenProxyMessage,
        RequestPasswordResetTokenProxyResponse
    > {
    view(): RequestResetTokenView
}
export function newRequestPasswordResetTokenProxy(
    post: Post<RequestPasswordResetTokenProxyMessage>,
): RequestPasswordResetTokenProxy {
    return new Proxy(post)
}

class Proxy
    extends WorkerAbstractProxy<
        RequestPasswordResetTokenProxyMessage,
        RequestPasswordResetTokenProxyResponse
    >
    implements RequestPasswordResetTokenProxy
{
    material: RequestPasswordResetTokenProxyMaterial

    constructor(post: Post<RequestPasswordResetTokenProxyMessage>) {
        super(post)
        this.material = {
            requestToken: this.method("requestToken", (message) => message),
        }
    }

    view(): RequestResetTokenView {
        return toApplicationView(
            initRequestResetTokenAction({
                requestToken: (fields, post) => this.material.requestToken.call({ fields }, post),
            }),
        )
    }
    resolve(response: RequestPasswordResetTokenProxyResponse): void {
        switch (response.method) {
            case "requestToken":
                this.material.requestToken.resolve(response)
                return
        }
    }
}

interface Post<M> {
    (message: M): void
}
