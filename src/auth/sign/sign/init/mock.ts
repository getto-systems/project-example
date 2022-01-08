import { mockSignViewTypeDetecter } from "../../router/init/mock"

import { SignActionShell } from "../action"

export function mockSignActionShell(url: URL): SignActionShell {
    return {
        detectViewType: mockSignViewTypeDetecter(url),
    }
}
