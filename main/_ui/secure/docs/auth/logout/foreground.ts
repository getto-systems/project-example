import { render, h } from "preact"

import { foregroundOutsideFeature } from "../../../../../../src/x_outside_feature/_ui/common"

import { newDocsView } from "../../../../../../src/docs/action_docs/init"

import { DocsUsecaseEntry } from "../../../../../../src/docs/action_docs/x_preact/usecase"

import { docsUsecaseContent } from "../../../../../../ui/vendor/getto-application/docs/helper"
import { docs_auth } from "../../../../../../src/auth/docs"

render(
    h(DocsUsecaseEntry, {
        view: newDocsView(foregroundOutsideFeature()),
        docs: docsUsecaseContent(docs_auth, "logout"),
    }),
    document.body,
)
