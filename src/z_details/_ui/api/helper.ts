import { ApiCommonError, ApiErrorResult, ApiInfraError } from "./data"
import { ApiFeature, ApiMethod, ApiRequest } from "./infra"

export function apiRequest(feature: ApiFeature, path: string, method: ApiMethod): ApiRequest {
    return {
        url: url(),
        options: {
            method,
            credentials: "include",
            headers: [["X-GETTO-EXAMPLE-NONCE", feature.nonce()]],
        },
    }

    function url(): string {
        const url = new URL(feature.serverURL)
        url.pathname = path

        return url.toString()
    }
}

export function apiStatusError(status: number): ApiErrorResult<ApiCommonError> {
    return { success: false, err: err() }

    function err(): ApiCommonError {
        switch (status) {
            case 401:
                return { type: "unauthorized" }

            case 409:
                return { type: "invalid-nonce" }

            default:
                return { type: "server-error" }
        }
    }
}
export function apiInfraError(err: unknown): ApiErrorResult<ApiInfraError> {
    return { success: false, err: { type: "infra-error", err: `${err}` } }
}
