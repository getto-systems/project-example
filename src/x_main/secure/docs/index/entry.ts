import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../x_outside_feature/common"
import { newDocsView } from "../../../../docs/init/resource"

import { Docs } from "../../../../docs/x_preact/docs"

import { docs_example } from "../../../../common/docs"

render(
    h(Docs, {
        view: newDocsView(newForegroundOutsideFeature()),
        title: "Example",
        docs: [docs_example],
    }),
    document.body,
)
