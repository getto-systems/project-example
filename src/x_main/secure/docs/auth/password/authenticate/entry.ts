import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../../../x_outside_feature/common"

import { newDocsView } from "../../../../../../docs/content/init/resource"

import { DocsUsecaseEntry } from "../../../../../../docs/content/x_preact/usecase"

import { docs_auth } from "../../../../../../auth/docs"
import { docsUsecase } from "../../../../../../z_vendor/getto-application/docs/helper"

render(
    h(DocsUsecaseEntry, {
        view: newDocsView(newForegroundOutsideFeature()),
        docs: docsUsecase(docs_auth, "password/authenticate"),
    }),
    document.body,
)
