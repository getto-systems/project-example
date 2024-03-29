import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../common/util/remote/detail/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../common/util/protobuf/helper"

import { RegisterAuthUserAccountRemoteResult, RegisterAuthUserAccountRemote } from "../infra"

import { RegisterAuthUserAccountRemoteError } from "../data"
import { AuthUserAccount } from "../../kernel/data"

export function newRegisterAuthUserAccountRemote(): RegisterAuthUserAccountRemote {
    return (fields) => fetchRemote(fields)
}

async function fetchRemote(fields: AuthUserAccount): Promise<RegisterAuthUserAccountRemoteResult> {
    const mock = false
    if (mock) {
        return { success: true, value: true }
    }

    try {
        const opts = fetchOptions({
            serverURL: env.apiServerURL,
            path: "/auth/user/account",
            method: "POST",
        })
        const response = await fetch(opts.url, {
            ...opts.options,
            body: encodeProtobuf(
                pb.auth.user.account.register.service.RegisterAuthUserAccountRequestPb,
                (message) => {
                    message.data = {
                        ...fields,
                        granted: Array.from(fields.granted),
                    }
                },
            ),
        })

        if (!response.ok) {
            return remoteCommonError(response.status)
        }

        const message = decodeProtobuf(
            pb.auth.user.account.register.service.RegisterAuthUserAccountResponsePb,
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
    err: pb.auth.user.account.register.service.RegisterAuthUserAccountErrorKindPb,
): RegisterAuthUserAccountRemoteError {
    switch (err) {
        case pb.auth.user.account.register.service.RegisterAuthUserAccountErrorKindPb
            .LOGIN_ID_ALREADY_REGISTERED:
            return { type: "login-id-already-registered" }

        case pb.auth.user.account.register.service.RegisterAuthUserAccountErrorKindPb.INVALID:
            return { type: "invalid" }
    }
}
