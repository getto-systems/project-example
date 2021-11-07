import {
    WorkerAbstractProxy,
    WorkerProxy,
} from "../../../../../../../../ui/vendor/getto-application/action/worker/foreground"

import {
    RequestResetTokenProfileProxyMaterial,
    RequestResetTokenProfileProxyMessage,
    RequestResetTokenProfileProxyResponse,
} from "./message"

import { RequestResetTokenProfileMaterial } from "../../action"

export interface RequestResetTokenProfileProxy
    extends WorkerProxy<
        RequestResetTokenProfileProxyMessage,
        RequestResetTokenProfileProxyResponse
    > {
    material(): RequestResetTokenProfileMaterial
}
export function newRequestResetTokenProfileProxy(
    post: Post<RequestResetTokenProfileProxyMessage>,
): RequestResetTokenProfileProxy {
    return new Proxy(post)
}

class Proxy
    extends WorkerAbstractProxy<
        RequestResetTokenProfileProxyMessage,
        RequestResetTokenProfileProxyResponse
    >
    implements RequestResetTokenProfileProxy
{
    proxy: RequestResetTokenProfileProxyMaterial

    constructor(post: Post<RequestResetTokenProfileProxyMessage>) {
        super(post)
        this.proxy = {
            requestToken: this.method("requestToken", (message) => message),
        }
    }

    material(): RequestResetTokenProfileMaterial {
        return {
            requestToken: (fields, post) => this.proxy.requestToken.call({ fields }, post),
        }
    }
    resolve(response: RequestResetTokenProfileProxyResponse): void {
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
