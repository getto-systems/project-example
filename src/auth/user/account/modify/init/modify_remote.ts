import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    generateNonce,
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../z_lib/ui/remote/init/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../z_vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { ModifyAuthUserAccountRemoteResult, ModifyAuthUserAccountRemote } from "../infra"

import { AuthUserAccountBasket } from "../../kernel/data"
import { ModifyAuthUserAccountFields } from "../data"

export function newModifyAuthUserAccountRemote(
    feature: RemoteOutsideFeature,
): ModifyAuthUserAccountRemote {
    return (user, fields) => fetchRemote(feature, user, fields)
}

async function fetchRemote(
    feature: RemoteOutsideFeature,
    user: AuthUserAccountBasket,
    _fields: ModifyAuthUserAccountFields,
): Promise<ModifyAuthUserAccountRemoteResult> {
    try {
        const mock = false
        if (mock) {
            return {
                success: true,
                value: user, // TODO fields の内容を反映した user を返す
            }
        }

        const opts = fetchOptions({
            serverURL: env.apiServerURL,
            path: "/auth/user/account",
            method: "PATCH",
            headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
        })
        const response = await fetch(opts.url, {
            ...opts.options,
            body: encodeProtobuf(
                // TODO Modify Request に変更してうまいことやる
                pb.auth.user.account.modify.service.OverrideLoginIdRequestPb,
                (message) => {
                    message.loginId = user.loginId
                },
            ),
        })

        if (!response.ok) {
            return remoteCommonError(response.status)
        }

        const message = decodeProtobuf(
            pb.auth.user.loginId.change.service.OverrideLoginIdResponsePb,
            await response.text(),
        )
        if (!message.success) {
            return errorResponse(message.err)
        }
        return {
            success: true,
            value: user, // TODO response から AuthUserAccountBasket を構築
        }
    } catch (err) {
        return remoteInfraError(err)
    }
}

function errorResponse(
    _err: pb.auth.user.loginId.change.service.OverrideLoginIdErrorKindPb,
): ModifyAuthUserAccountRemoteResult {
    // TODO 正しいエラー処理にする
    return { success: false, err: { type: "invalid-granted-role" } }
    // switch (err) {
    //     case pb.auth.user.loginId.change.service.OverrideLoginIdErrorKindPb.INVALID_LOGIN_ID:
    //         return { success: false, err: { type: "invalid-login-id" } }

    //     case pb.auth.user.loginId.change.service.OverrideLoginIdErrorKindPb.ALREADY_REGISTERED:
    //         return { success: false, err: { type: "already-registered" } }
    // }
}
