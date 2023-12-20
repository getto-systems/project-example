import { mockGetScriptPathShell } from "../../../../../sign/get_script_path/detail/mock"

import { detectResetToken } from "../convert"

import { ResetPasswordShell } from "../action"

export function mockResetPasswordShell(currentURL: URL): ResetPasswordShell {
    return {
        ...mockGetScriptPathShell(currentURL),
        detectResetToken: () => detectResetToken(currentURL),
    }
}
