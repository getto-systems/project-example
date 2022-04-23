import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import {
    generateNonce,
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../z_lib/ui/remote/init/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../z_vendor/protobuf/helper"

import { UnregisterAuthUserAccountRemoteResult, UnregisterAuthUserAccountRemote } from "../infra"

import { UnregisterAuthUserAccountRemoteError } from "../data"
import { LoginId } from "../../../login_id/kernel/data"

export function newUnregisterAuthUserAccountRemote(
    feature: RemoteOutsideFeature,
): UnregisterAuthUserAccountRemote {
    return (user) => fetchRemote(feature, user)
}

async function fetchRemote(
    feature: RemoteOutsideFeature,
    user: Readonly<{ loginId: LoginId }>,
): Promise<UnregisterAuthUserAccountRemoteResult> {
    // TODO つなぐ
    const mock = true
    if (mock) {
        return { success: true, value: true }
    }

    try {
        const opts = fetchOptions({
            serverURL: env.apiServerURL,
            path: "/auth/user/account",
            method: "DELETE",
            headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
        })
        const response = await fetch(opts.url, {
            ...opts.options,
            body: encodeProtobuf(
                pb.auth.user.account.unregister.service.UnregisterAuthUserAccountRequestPb,
                (message) => {
                    message.loginId = user.loginId
                },
            ),
        })

        if (!response.ok) {
            return remoteCommonError(response.status)
        }

        const message = decodeProtobuf(
            pb.auth.user.account.unregister.service.UnregisterAuthUserAccountResponsePb,
            await response.text(),
        )
        if (!message.success) {
            return { success: false, err: errorResponse(message.err) }
        }
        return { success: true, value: true }
    } catch (err) {
        return remoteInfraError(err)
    }
}

function errorResponse(
    err: pb.auth.user.account.unregister.service.UnregisterAuthUserAccountErrorKindPb,
): UnregisterAuthUserAccountRemoteError {
    switch (err) {
        case pb.auth.user.account.unregister.service.UnregisterAuthUserAccountErrorKindPb.NOT_FOUND:
            return { type: "not-found" }

        case pb.auth.user.account.unregister.service.UnregisterAuthUserAccountErrorKindPb.INVALID:
            return { type: "invalid" }
    }
}
