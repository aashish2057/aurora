import type { NextApiRequest, NextApiResponse } from "next"

import { pb } from "../../pocketbase/pocketbase"
import { client } from "../plaid"

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method === "POST") {
    try {
      const publicToken = req.body.publicToken
      const response = await client.itemPublicTokenExchange({
        public_token: publicToken,
      })
      const data = {
        access_token: response.data.access_token,
        item_id: response.data.item_id,
      }

      const record = await pb.collection("access_tokens").create(data)
      console.log(record)

      res.status(200).json({ success: "access granted" })
    } catch (error) {
      console.log(error)
      res.status(400).json({ erorr: error })
    }
  } else {
    res.status(405).json({ error: "Invalid HTTP Method" })
  }
}
