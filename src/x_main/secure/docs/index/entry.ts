import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../x_outside_feature/common"
import { newDocsResource } from "../../../../docs/detail/resource"

import { Docs } from "../../../../docs/x_preact/docs"

import { docs_example } from "../../../../core/docs"

render(
    h(Docs, {
        ...newDocsResource(newForegroundOutsideFeature()),
        title: "Example",
        docs: [docs_example],
    }),
    document.body,
)
