import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../../../../src/x_outside_feature/common"

import { newDocsView } from "../../../../../../../src/docs/action_docs/init/resource"

import { DocsUsecaseEntry } from "../../../../../../../src/docs/action_docs/x_preact/usecase"

import { docs_auth } from "../../../../../../../src/auth/docs"
import { docsUsecase } from "../../../../../../../ui/vendor/getto-application/docs/helper"

render(
    h(DocsUsecaseEntry, {
        view: newDocsView(newForegroundOutsideFeature()),
        docs: docsUsecase(docs_auth, "ticket/logout"),
    }),
    document.body,
)
