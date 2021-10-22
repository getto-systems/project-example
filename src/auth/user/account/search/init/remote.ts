import { env } from "../../../../../y_environment/ui/env"
// import pb from "../../../../../y_protobuf/proto.js"

import {
    generateNonce,
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../z_lib/ui/remote/helper"
// import { decodeProtobuf, encodeProtobuf } from "../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { SearchUserAccountRemote } from "../infra"
import { LoginID } from "../../../login_id/input/data"
import { GrantedRoles } from "../../../../ticket/kernel/data"
import { ticker } from "../../../../../z_lib/ui/timer/helper"

export function newSearchUserAccountRemote(feature: RemoteOutsideFeature): SearchUserAccountRemote {
    return async (_fields) => {
        try {
            const mock = true
            if (mock) {
                await ticker({ wait_millisecond: 3000 }, () => null)
                return {
                    success: true,
                    value: {
                        page: { offset: 0, limit: 10, all: 25 },
                        users: [
                            {
                                loginID: "admin" as LoginID,
                                grantedRoles: ["dev-docs"] as GrantedRoles,
                            },
                        ],
                    },
                }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/auth/user/account/search",
                method: "GET",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            // TODO 検索パラメータを GET で送信(URL に含めて渡す)
            const response = await fetch(opts.url, {
                ...opts.options,
                // body: encodeProtobuf(
                //     pb.auth.user.password.api.AuthenticatePasswordApiRequestPb,
                //     (message) => {
                //         message.loginId = fields.loginID
                //         message.password = fields.password
                //     },
                // ),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
            }
            return {
                success: true,
                value: { page: { offset: 0, limit: 1000, all: 245 }, users: [] },
            }

            // TODO response を受け取る
            // const result = decodeProtobuf(
            //     pb.auth.user.password.api.AuthenticatePasswordApiResponsePb,
            //     await response.text(),
            // )
            // if (!result.success) {
            //     return { success: false, err: { type: "invalid-password" } }
            // }
            // return {
            //     success: true,
            //     value: convertAuthRemote(clock, { roles: result.value?.roles || [] }),
            // }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
