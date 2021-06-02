import { env } from "../../../../../../y_environment/env"
import { GetMenuBadgeResult_pb } from "../../../../y_protobuf/api_pb.js"

import {
    remoteFeature,
    convertRemote,
} from "../../../../../../../ui/vendor/getto-application/infra/remote/helper"
import { decodeProtobuf } from "../../../../../../../ui/vendor/protobuf/helper"
import {
    apiInfraError,
    apiRequest,
    apiStatusError,
} from "../../../../../../z_details/_ui/api/helper"

import { RemoteOutsideFeature } from "../../../../../../../ui/vendor/getto-application/infra/remote/infra"
import { GetMenuBadgeRemotePod } from "../../../infra"

import { ApiCommonError, ApiResult } from "../../../../../../z_details/_ui/api/data"

export function newGetMenuBadgeRemote(feature: RemoteOutsideFeature): GetMenuBadgeRemotePod {
    type GetMenuResult = ApiResult<MenuBadgeItem[], ApiCommonError>
    type MenuBadgeItem = Readonly<{ path: string; count: number }>

    return convertRemote(
        async (): Promise<GetMenuResult> => {
            try {
                const mock = true
                if (mock) {
                    // TODO api の実装が終わったらつなぐ
                    return { success: true, value: [] }
                }

                const request = apiRequest(
                    remoteFeature(env.apiServerURL, feature),
                    "/outline/menu/badge",
                    "GET",
                )
                const response = await fetch(request.url, request.options)

                if (!response.ok) {
                    return apiStatusError(response.status)
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
                return apiInfraError(err)
            }
        },
    )
}
