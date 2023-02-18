import { NavItem } from "@/types/nav"

interface SiteConfig {
  name: string
  description: string
  mainNav: NavItem[]
  links: {
    github: string
  }
}

export const siteConfig: SiteConfig = {
  name: "Aurora",
  description: "Financial Dashbaord",
  mainNav: [
    {
      title: "Home",
      href: "/",
    },
  ],
  links: {
    github: "https://github.com/aashish2057/aurora",
  },
}
