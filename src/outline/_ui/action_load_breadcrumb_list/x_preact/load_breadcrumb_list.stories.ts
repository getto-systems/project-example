import { h } from "preact"

import { enumKeys, storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import { LoadBreadcrumbListComponent } from "./load_breadcrumb_list"

import { mockBreadcrumbList, mockLoadBreadcrumbListCoreAction } from "../core/mock"

enum LoadEnum {
    "home",
    "empty",
}

export default {
    title: "library/Outline/Menu/Load Breadcrumb List",
    argTypes: {
        load: {
            control: { type: "select", options: enumKeys(LoadEnum) },
        },
    },
}

type MockProps = Readonly<{
    load: keyof typeof LoadEnum
    label: string
}>
const template = storyTemplate<MockProps>((props) => {
    return h(LoadBreadcrumbListComponent, {
        breadcrumbList: mockLoadBreadcrumbListCoreAction(mockBreadcrumbList(props.label)),
    })
})

export const LoadBreadcrumbList = template({ load: "home", label: "ホーム" })
