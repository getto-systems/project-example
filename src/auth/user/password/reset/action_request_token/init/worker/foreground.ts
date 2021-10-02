import {
    WorkerAbstractProxy,
    WorkerProxy,
} from "../../../../../../../../ui/vendor/getto-application/action/worker/foreground"

import {
    RequestResetTokenProxyMaterial,
    RequestResetTokenProxyMessage,
    RequestResetTokenProxyResponse,
} from "./message"

import { RequestResetTokenMaterial } from "../../action"

export interface RequestResetTokenProxy
    extends WorkerProxy<
        RequestResetTokenProxyMessage,
        RequestResetTokenProxyResponse
    > {
    material(): RequestResetTokenMaterial
}
export function newRequestResetTokenProxy(
    post: Post<RequestResetTokenProxyMessage>,
): RequestResetTokenProxy {
    return new Proxy(post)
}

class Proxy
    extends WorkerAbstractProxy<
        RequestResetTokenProxyMessage,
        RequestResetTokenProxyResponse
    >
    implements RequestResetTokenProxy
{
    proxy: RequestResetTokenProxyMaterial

    constructor(post: Post<RequestResetTokenProxyMessage>) {
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
    resolve(response: RequestResetTokenProxyResponse): void {
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
