import { mockSignViewTypeDetecter } from "../../router/detail/mock"

import { SignActionShell } from "../action"

export function mockSignActionShell(url: URL): SignActionShell {
    return {
        detectViewType: mockSignViewTypeDetecter(url),
    }
}
