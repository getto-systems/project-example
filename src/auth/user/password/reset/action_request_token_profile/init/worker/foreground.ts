import {
    WorkerAbstractProxy,
    WorkerProxy,
} from "../../../../../../../../ui/vendor/getto-application/action/worker/foreground"

import {
    RequestPasswordResetTokenProfileProxyMaterial,
    RequestPasswordResetTokenProfileProxyMessage,
    RequestPasswordResetTokenProfileProxyResponse,
} from "./message"

import { RequestResetTokenProfileMaterial } from "../../action"

export interface RequestPasswordResetTokenProfileProxy
    extends WorkerProxy<
        RequestPasswordResetTokenProfileProxyMessage,
        RequestPasswordResetTokenProfileProxyResponse
    > {
    material(): RequestResetTokenProfileMaterial
}
export function newRequestPasswordResetTokenProfileProxy(
    post: Post<RequestPasswordResetTokenProfileProxyMessage>,
): RequestPasswordResetTokenProfileProxy {
    return new Proxy(post)
}

class Proxy
    extends WorkerAbstractProxy<
        RequestPasswordResetTokenProfileProxyMessage,
        RequestPasswordResetTokenProfileProxyResponse
    >
    implements RequestPasswordResetTokenProfileProxy
{
    proxy: RequestPasswordResetTokenProfileProxyMaterial

    constructor(post: Post<RequestPasswordResetTokenProfileProxyMessage>) {
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
    resolve(response: RequestPasswordResetTokenProfileProxyResponse): void {
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
