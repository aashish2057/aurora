import type { NextApiRequest, NextApiResponse } from "next"
import accessToken from "@/pages/api/pocketbase/accessToken"
import updateTransactions from "@/pages/api/pocketbase/updateTransactions"

import { client } from "../plaid"

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method === "GET") {
    try {
      const result = await accessToken()

      const response = await client.transactionsSync({
        access_token: result.access_token,
        cursor: undefined,
      })
      updateTransactions(response.data)
      res.status(200).json(response.data)
    } catch (error) {
      console.log(error)
      res.status(400).json({ erorr: error })
    }
  } else {
    res.status(405).json({ error: "Invalid HTTP Method" })
  }
}
