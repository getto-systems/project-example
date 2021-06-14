import { env } from "../../../../../../y_environment/_ui/env"
import { GetMenuBadgeResult_pb } from "../../../../y_protobuf/api_pb.js"

import {
    convertRemote,
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../z_details/_ui/remote/helper"
import { decodeProtobuf } from "../../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../../z_details/_ui/remote/feature"

import { GetMenuBadgeRemotePod } from "../../../infra"

import { ApiCommonError, ApiResult } from "../../../../../../z_details/_ui/api/data"

export function newGetMenuBadgeRemote(feature: RemoteOutsideFeature): GetMenuBadgeRemotePod {
    type GetMenuResult = ApiResult<MenuBadgeItem[], ApiCommonError>
    type MenuBadgeItem = Readonly<{ path: string; count: number }>

    return convertRemote(async (): Promise<GetMenuResult> => {
        try {
            const mock = true
            if (mock) {
                // TODO api の実装が終わったらつなぐ
                return { success: true, value: [] }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/outline/menu/badge",
                method: "GET",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, opts.options)

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const result = decodeProtobuf(GetMenuBadgeResult_pb, await response.text())
            return {
                success: true,
                value: result.badge.map((item) => ({
                    path: item.path || "",
                    count: item.count || 0,
                })),
            }
        } catch (err) {
            return remoteInfraError(err)
        }
    })
}
