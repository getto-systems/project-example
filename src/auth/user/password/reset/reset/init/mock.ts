import { mockGetScriptPathShell } from "../../../../../sign/get_script_path/init/mock"

import { detectResetToken } from "../../../input/convert"

import { ResetPasswordShell } from "../action"

export function mockResetPasswordShell(currentURL: URL): ResetPasswordShell {
    return {
        ...mockGetScriptPathShell(currentURL),
        detectResetToken: () => detectResetToken(currentURL),
    }
}
