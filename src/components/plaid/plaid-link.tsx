import React, { useEffect, useState } from "react"
import { usePlaidLink } from "react-plaid-link"

import { Icons } from "../icons"
import { buttonVariants } from "../ui/button"

const PlaidLink = () => {
  const [linkToken, setLinkToken] = useState(null)

  useEffect(() => {
    async function generateToken() {
      const response = await fetch("/api/plaid/create-link-token", {
        method: "POST",
      })
      const data = await response.json()
      setLinkToken((t) => (t = data.link_token))
    }

    generateToken()
  }, [])

  const onSuccess = React.useCallback(
    async (publicToken: any, metadata: any) => {
      const response = await fetch("/api/plaid/set-access-token", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ publicToken }),
      })
    },
    []
  )

  const config: Parameters<typeof usePlaidLink>[0] = {
    token: linkToken!,
    onSuccess,
  }

  const { open, ready } = usePlaidLink(config)

  return (
    <button onClick={() => open()}>
      <div
        className={buttonVariants({
          size: "sm",
          variant: "ghost",
          className: "text-slate-700 dark:text-slate-400",
        })}
      >
        <Icons.plus className="h-5 w-5" />
        <span className="sr-only">GitHub</span>
      </div>
    </button>
  )
}

export { PlaidLink }
