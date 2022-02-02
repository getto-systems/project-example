export type SiteInfo = Readonly<{
    brand: string
    title: string
    subTitle: string
}>

export const siteInfo: SiteInfo = {
    brand: "GETTO",
    title: "Example",
    subTitle: "code templates",
}

export const copyright = "GETTO.systems" as const
export const poweredBy = ["LineIcons", "みんなの文字"] as const
