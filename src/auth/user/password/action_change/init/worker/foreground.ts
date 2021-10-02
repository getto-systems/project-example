import {
    WorkerAbstractProxy,
    WorkerProxy,
} from "../../../../../../../ui/vendor/getto-application/action/worker/foreground"

import {
    ChangePasswordProxyMaterial,
    ChangePasswordProxyMessage,
    ChangePasswordProxyResponse,
} from "./message"

import { ChangePasswordMaterial } from "../../action"

export interface RequestResetTokenProfileProxy
    extends WorkerProxy<ChangePasswordProxyMessage, ChangePasswordProxyResponse> {
    material(): ChangePasswordMaterial
}
export function newChangePasswordProxy(
    post: Post<ChangePasswordProxyMessage>,
): RequestResetTokenProfileProxy {
    return new Proxy(post)
}

class Proxy
    extends WorkerAbstractProxy<ChangePasswordProxyMessage, ChangePasswordProxyResponse>
    implements RequestResetTokenProfileProxy
{
    proxy: ChangePasswordProxyMaterial

    constructor(post: Post<ChangePasswordProxyMessage>) {
        super(post)
        this.proxy = {
            change: this.method("change", (message) => message),
        }
    }

    material(): ChangePasswordMaterial {
        return {
            change: (fields, post) => this.proxy.change.call({ fields }, post),
        }
    }
    resolve(response: ChangePasswordProxyResponse): void {
        switch (response.method) {
            case "change":
                this.proxy.change.resolve(response)
                return
        }
    }
}

interface Post<M> {
    (message: M): void
}
