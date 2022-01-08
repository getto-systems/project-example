import { detectSignViewType } from "../convert"

import { SignViewTypeDetecter } from "../infra"

export function mockSignViewTypeDetecter(currentURL: URL): SignViewTypeDetecter {
    return () => detectSignViewType(currentURL)
}
