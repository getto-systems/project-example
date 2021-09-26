import {
    WorkerAbstractProxy,
    WorkerProxy,
} from "../../../../../../../ui/vendor/getto-application/action/worker/foreground"

import {
    RequestPasswordResetTokenProxyMaterial,
    RequestPasswordResetTokenProxyMessage,
    RequestPasswordResetTokenProxyResponse,
} from "./message"

import { RequestResetTokenMaterial } from "../../action"

export interface RequestPasswordResetTokenProxy
    extends WorkerProxy<
        RequestPasswordResetTokenProxyMessage,
        RequestPasswordResetTokenProxyResponse
    > {
    material(): RequestResetTokenMaterial
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
    proxy: RequestPasswordResetTokenProxyMaterial

    constructor(post: Post<RequestPasswordResetTokenProxyMessage>) {
        super(post)
        this.proxy = {
            requestToken: this.method("requestToken", (message) => message),
        }
    }

    material(): RequestResetTokenMaterial {
        return {
            requestToken: (fields, post) => this.proxy.requestToken.call({ fields }, post),
        }
    }
    resolve(response: RequestPasswordResetTokenProxyResponse): void {
        switch (response.method) {
            case "requestToken":
                this.proxy.requestToken.resolve(response)
                return
        }
    }
}

interface Post<M> {
    (message: M): void
}
