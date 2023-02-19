
import { ThemeToggle } from "@/components/theme-toggle"
import { PlaidLink } from "./plaid/plaid-link"

export function SiteHeader() {
  return (
    <header className="sticky top-0 z-40 w-full border-b border-b-slate-200 bg-white dark:border-b-slate-700 dark:bg-slate-900">
      <div className="container flex h-16 items-center space-x-4 sm:justify-between sm:space-x-0">
        <div className="font-bold text-xl pr-5">Aurora</div>
        <PlaidLink />
        <div className="flex flex-1 items-center justify-end space-x-4">
          <nav className="flex items-center space-x-1">
            <ThemeToggle />
          </nav>
        </div>
      </div>
    </header>
  )
}
