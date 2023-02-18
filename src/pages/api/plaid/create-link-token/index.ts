import type { NextApiRequest, NextApiResponse } from "next"
import { CountryCode, Products } from "plaid"

import { client } from "../plaid"

const configs = {
  client_name: "Insert Client name here",
  country_codes: [CountryCode.Us],
  language: "en",
  user: {
    client_user_id: "unique_user_id",
  },
  products: [Products.Liabilities, Products.Transactions],
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method === "POST") {
    try {
      const response = await client.linkTokenCreate(configs)
      const data = response.data
      res.status(200).json(data)
    } catch (error) {
      console.log(error)
      res.status(400).json({ erorr: error })
    }
  } else {
    res.status(405).json({ error: "Invalid HTTP Method" })
  }
}
